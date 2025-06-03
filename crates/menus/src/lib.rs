mod fmg;
mod gui;

use crash_handler::{
    CrashContext, CrashEventResult, CrashHandler, make_crash_event,
};
use pelite::pe::PeObject;
use std::{
    fs::File,
    sync::{Arc, Mutex, atomic::AtomicUsize},
    time::Duration,
};

use crate::gui::HookGui;

use eldenring_util::{program::Program, system::wait_for_system_init};
use fmg::FmgCategories;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use last_weapon::retour::record_rax_trampoline_data_with_data_ptr;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc,
};

const WILD_STRIKES_ID: u32 = 110;

#[derive(Clone, Copy)]
struct PtrWrapper(*mut u8);

unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}

#[unsafe(no_mangle)]
/// # DllMain Entry Point
///
/// This is the entry point for the DLL. It is called by the operating system when the DLL is loaded or unloaded.
///
/// ## Parameters
/// - `_hmodule`: A handle to the DLL module. This parameter is unused in this implementation.
/// - `reason`: The reason code for the call. This implementation only handles `DLL_PROCESS_ATTACH` (value `1`).
///
/// ## Behavior
/// When the `reason` is `DLL_PROCESS_ATTACH`, the following actions are performed:
/// 1. The `setup` function is called to initialize logging, crash handling, and other setup tasks.
/// 2. A new thread is spawned to:
///    - Wait for the system to initialize using `wait_for_system_init`.
///    - Call the `init` function to perform the main DLL initialization logic.
///
/// ## Safety
/// This function is marked as `unsafe` because it interacts with low-level system APIs and performs operations
/// that require careful handling, such as spawning threads and modifying memory.
///
/// ## Returns
/// Always returns `true` to indicate successful execution.
/// ```
pub unsafe extern "C" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _: *mut std::ffi::c_void,
) {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        init_logging_and_crash_handler().unwrap();
        let program: Program<'_> = Program::current();

        let message_replacements = setup_message_replacements();

        std::thread::spawn(move || {
            let (original_instructions, get_weapon_hook, data_address) =
                prepare_hook_context(program);
            build_gui_instance(
                hmodule,
                message_replacements,
                original_instructions,
                get_weapon_hook,
                data_address,
            );
        });
    }
}

fn prepare_hook_context(
    program: Program<'_>,
) -> ([u8; 5], Arc<Mutex<PtrWrapper>>, AtomicUsize) {
    let (write_data_addr, original_instructions, get_weapon_hook) =
        initialize_program_jump_hook(program);
    let atomic_usize_data_addr = AtomicUsize::new(write_data_addr as usize);
    (original_instructions, get_weapon_hook, atomic_usize_data_addr)
}

fn build_gui_instance(
    hmodule: HINSTANCE,
    message_replacements: Arc<MessageReplacements>,
    original_instructions: [u8; 5],
    get_weapon_hook: Arc<Mutex<PtrWrapper>>,
    weapon_data_address: AtomicUsize,
) {
    let get_weapon_hook_copy = get_weapon_hook.clone();
    let gui =
        HookGui::new(message_replacements, weapon_data_address, move || {
            unsafe {
                let _ = unhook(&get_weapon_hook, original_instructions);
            };
        });

    let result = hudhook::Hudhook::builder()
        .with::<ImguiDx12Hooks>(gui)
        .with_hmodule(hmodule)
        .build()
        .apply();

    if let Err(_e) = result {
        unsafe {
            let _ = unhook(&get_weapon_hook_copy, original_instructions);
        };
    }
}

fn setup_message_replacements() -> Arc<MessageReplacements> {
    let message_replacements: MessageReplacements = Mutex::new(Vec::new());

    let message_replacements = create_replacement_message(
        message_replacements,
        FmgCategories::ArtsCaption,
        WILD_STRIKES_ID,
    );
    Arc::new(message_replacements)
}

fn initialize_program_jump_hook(
    program: Program<'_>,
) -> (*const u8, [u8; 5], Arc<Mutex<PtrWrapper>>) {
    tracing::info!("Running new thread for hook setup");
    let duration: Duration = Duration::from_millis(5000);
    wait_for_system_init(&program, duration)
        .expect("System initialization timed out");
    let base_address = match program {
        Program::Mapping(pe_view) => pe_view.image().as_ptr(),
        Program::File(pe_file) => pe_file.image().as_ptr(),
    };

    let (write_data_addr, jump_instruction_to_buffer) =
        match rax_read_ptr_jump_instructions(base_address) {
            Some(value) => value,
            None => unreachable!("Failed to get jump instructions"),
        };
    let (original_instructions, get_weapon_hook) =
        replace_instructions_with_jump(
            base_address,
            jump_instruction_to_buffer,
        );
    (write_data_addr, original_instructions, get_weapon_hook)
}

fn replace_instructions_with_jump(
    base_address: *const u8,
    jump_instruction_to_buffer: [u8; 5],
) -> ([u8; 5], Arc<Mutex<PtrWrapper>>) {
    let mut original_instructions: [u8; 5] = [0; 5];
    let get_weapon_hook_ptr_address = unsafe { base_address.offset(0x7c045d) };
    // Replace the 5 byte instruction at the address with a jump to our hook
    let get_weapon_hook_ptr = get_weapon_hook_ptr_address as *mut u8;
    unsafe {
        tracing::info!(
            "Weapon hook pointer address: {:?}",
            get_weapon_hook_ptr
        );

        // Write the jump instruction to the address
        for (i, &byte) in jump_instruction_to_buffer.iter().enumerate() {
            original_instructions[i] = *get_weapon_hook_ptr.add(i);
            *get_weapon_hook_ptr.add(i) = byte;
        }
    };
    let get_weapon_hook =
        Arc::new(Mutex::new(PtrWrapper(get_weapon_hook_ptr_address as _)));
    (original_instructions, get_weapon_hook)
}

unsafe fn unhook(
    get_weapon_hook_ptr: &Arc<Mutex<PtrWrapper>>,
    original_instructions: [u8; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, &byte) in original_instructions.iter().enumerate() {
        unsafe { *get_weapon_hook_ptr.lock().unwrap().0.add(i) = byte };
    }
    hudhook::eject();
    Ok(())
}

fn init_logging_and_crash_handler() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    init_crash_handler();

    Ok(())
}

fn init_crash_handler() {
    #[allow(unsafe_code)]
    let handler = CrashHandler::attach(unsafe {
        make_crash_event(move |context: &CrashContext| {
            tracing::error!(
                "Exception: {:x} at {:x}",
                context.exception_code,
                (*(*context.exception_pointers).ExceptionRecord)
                    .ExceptionAddress as usize
            );
            hudhook::eject();

            CrashEventResult::Handled(true)
        })
    })
    .unwrap();
    std::mem::forget(handler);
}

fn init_logging() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let log_file = File::create("./er_menu_mod.log")?;
    let subscriber =
        tracing_subscriber::fmt().with_writer(Mutex::new(log_file)).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("Application panicked: {}", panic_info);
    }));
    Ok(())
}

fn rax_read_ptr_jump_instructions(
    base_address: *const u8,
) -> Option<(*const u8, [u8; 5])> {
    // Find a memory region close to the target
    let target_addr = unsafe { base_address.add(0x7c045d) };
    let return_addr = unsafe { base_address.add(0x7c0462) };

    // Try multiple memory locations, starting with the closest one
    let mut exec_mem = std::ptr::null_mut();

    // Try different strategies to find suitable memory
    let instructions =
        record_rax_trampoline_data_with_data_ptr(None, Some(return_addr));
    let (_data_addr_offset, hook_code) = match instructions {
        Ok(code) => code,
        Err(e) => {
            tracing::error!("Failed to build hook sequence: {}", e);
            return None;
        }
    };
    for attempt in 0..40 {
        let offset = if attempt == 0 {
            0 // First try the exact location
        } else {
            attempt * 0x100000 // Then try at 1MB intervals
        };

        let try_addr: *const u8 = match attempt % 2 {
            0 => {
                ((target_addr as usize).wrapping_add(offset) & !0xFFFFF)
                    as *const _
            } // Try above
            _ => {
                ((target_addr as usize).wrapping_sub(offset) & !0xFFFFF)
                    as *const _
            } // Try below
        };

        exec_mem = unsafe {
            VirtualAlloc(
                Some(try_addr as *mut std::ffi::c_void),
                hook_code.len(),
                MEM_COMMIT | MEM_RESERVE,
                PAGE_EXECUTE_READWRITE,
            )
        };

        if !exec_mem.is_null() {
            tracing::info!(
                "Successfully allocated memory at 0x{:X}",
                exec_mem as usize
            );
            break;
        }
    }

    if exec_mem.is_null() {
        tracing::error!(
            "Failed to allocate executable memory after multiple attempts"
        );
        return None;
    }

    let sequence = record_rax_trampoline_data_with_data_ptr(
        Some(exec_mem as *const u8),
        Some(return_addr),
    );
    let (data_addr_offset, instructions) = match sequence {
        // TODO: integrate this into the return value
        Ok(code) => code,
        Err(e) => {
            tracing::error!("Failed to build hook sequence: {}", e);
            return None;
        }
    };
    // Rest of the function remains the same
    unsafe {
        std::ptr::copy_nonoverlapping(
            instructions.as_ptr(),
            exec_mem as *mut u8,
            instructions.len(),
        );
    }

    let hook_addr = exec_mem as usize;
    tracing::info!("New hook code address: 0x{:X}", hook_addr);

    let rel_offset = ((hook_addr as isize) - (target_addr as isize + 5)) as i32;

    let jump_instruction = [
        0xE9, // JMP instruction
        (rel_offset & 0xFF) as u8,
        ((rel_offset >> 8) & 0xFF) as u8,
        ((rel_offset >> 16) & 0xFF) as u8,
        ((rel_offset >> 24) & 0xFF) as u8,
    ];

    static mut EXEC_MEM_PTR: *mut std::ffi::c_void = std::ptr::null_mut();
    unsafe { EXEC_MEM_PTR = exec_mem };

    Some((
        ((exec_mem as usize).wrapping_add(data_addr_offset)) as *const u8,
        jump_instruction,
    ))
}
struct LimitedList<T>(Vec<T>);

impl<T> LimitedList<T> {
    const MAX_LENGTH: usize = 0x257;
    fn new(items: Vec<T>) -> Result<Self, String> {
        if items.len() > Self::MAX_LENGTH {
            Err(String::from("List length exceeds limit"))
        } else {
            Ok(Self(items))
        }
    }

    fn extend(&mut self, items: Vec<T>) {
        self.0.extend(items);
        if self.0.len() > Self::MAX_LENGTH {
            tracing::warn!("List length exceeds limit: {}", self.0.len());
        }
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }
}
type MessageReplacements =
    Mutex<Vec<(FmgCategories, u32, Arc<Mutex<LimitedList<u16>>>)>>;

fn create_replacement_message(
    message_replacements: MessageReplacements,
    category: FmgCategories,
    id: u32,
) -> MessageReplacements {
    let replacement_text = r"00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f 80 81 82 83 84 85 86 87 88 89 8a 8b 8c 8d 8e 8f 90 91 92 93 94 95 96 97 98 99 9a 9b 9c 9d 9e 9f a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 aa ab ac ad ae af b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 ba bb bc bd be bf c0 c1 c2 c3 c4 c5 c6 c7";
    let buffer: Vec<u16> = replacement_text.encode_utf16().collect();
    let buffer_box = buffer.into_boxed_slice();
    let replacement_message_ptr = replacement_message_ptr(buffer_box);
    message_replacements.lock().unwrap().push((
        category,
        id,
        replacement_message_ptr,
    ));
    message_replacements
}

fn replacement_message_ptr(
    buffer_box: Box<[u16]>,
) -> Arc<Mutex<LimitedList<u16>>> {
    let replacement_msg_ptr = Box::leak(buffer_box);
    let vec_msg = replacement_msg_ptr.to_vec();
    let message = String::from_utf16_lossy(vec_msg.as_slice());
    let expectation_message =
        format!("Replacement message exceeds length limit: {:?}", message);
    let expectation_message = expectation_message.as_str();
    let limited_list =
        LimitedList::new(vec_msg.clone()).expect(expectation_message);

    Arc::new(Mutex::new(limited_list))
}

mod test {
    #[test]
    fn test_create_replacement_message() {
        let message_replacements = std::sync::Mutex::new(Vec::new());
        let category = crate::fmg::FmgCategories::ArtsCaption;
        let id = crate::WILD_STRIKES_ID;

        let result = crate::create_replacement_message(
            message_replacements,
            category,
            id,
        );
        assert!(
            !result.lock().unwrap().is_empty(),
            "Message replacements should not be empty"
        );
    }
}
