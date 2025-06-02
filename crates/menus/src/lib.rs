mod fmg;
mod gui;

use crash_handler::{
    CrashContext, CrashEventResult, CrashHandler, make_crash_event,
};
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
        setup().unwrap();
        let program: Program<'_> = Program::current();

        let message_replacements: MessageReplacements = Mutex::new(Vec::new());

        let message_replacements = create_replacement_message(
            message_replacements,
            FmgCategories::ArtsCaption,
            WILD_STRIKES_ID,
        );
        let message_replacements = Arc::new(message_replacements);

        std::thread::spawn(move || {
            tracing::info!("Running new thread for hook setup");
            let duration: Duration = Duration::from_millis(5000);
            wait_for_system_init(&program, duration)
                .expect("System initialization timed out");
            let (data_addr, jump_instruction_to_buffer) =
                match jump_instructions() {
                    Some(value) => value,
                    None => unreachable!("Failed to get jump instructions"),
                };
            let base_address: *const u8 = 0x140000000 as *const u8;
            let mut original_instructions: [u8; 5] = [0; 5];
            let get_weapon_hook_ptr_addrss =
                unsafe { base_address.offset(0x7c045d) };
            // Replace the 5 byte instruction at the address with a jump to our hook
            let get_weapon_hook_ptr = get_weapon_hook_ptr_addrss as *mut u8;
            unsafe {
                tracing::info!(
                    "Weapon hook pointer address: {:?}",
                    get_weapon_hook_ptr
                );

                // Write the jump instruction to the address
                for (i, &byte) in jump_instruction_to_buffer.iter().enumerate()
                {
                    original_instructions[i] = *get_weapon_hook_ptr.add(i);
                    *get_weapon_hook_ptr.add(i) = byte;
                }
            };
            let arc_wrapped_ptr = Arc::new(Mutex::new(PtrWrapper(
                get_weapon_hook_ptr_addrss as _,
            )));
            let atomic_usize_data_addr = AtomicUsize::new(data_addr as usize);
            let arc_wrapped_ptr_clone = arc_wrapped_ptr.clone();
            let gui = HookGui::new(
                message_replacements,
                atomic_usize_data_addr,
                move || {
                    unsafe {
                        let _ = unhook(
                            &arc_wrapped_ptr_clone,
                            original_instructions,
                        );
                    };
                },
            );

            let result = hudhook::Hudhook::builder()
                .with::<ImguiDx12Hooks>(gui)
                .with_hmodule(hmodule)
                .build()
                .apply();

            if let Err(_e) = result {
                unsafe {
                    let _ = unhook(&arc_wrapped_ptr, original_instructions);
                };
            }
        });
    }
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

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = File::create("./er_menu_mod.log")?;
    let subscriber =
        tracing_subscriber::fmt().with_writer(Mutex::new(log_file)).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("Application panicked: {}", panic_info);
    }));

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

    Ok(())
}

fn jump_instructions() -> Option<(*const u8, [u8; 5])> {
    // Get your hook code bytes

    // Find a memory region close to the target
    let base_address: usize = 0x140000000;
    let target_addr = base_address + 0x7c045d;
    let return_addr = base_address + 0x7c0462;

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

        let try_addr = match attempt % 2 {
            0 => (target_addr + offset) & !0xFFFFF, // Try above
            _ => (target_addr - offset) & !0xFFFFF, // Try below
        };

        tracing::info!(
            "Attempt {}: trying to allocate at 0x{:X}",
            attempt,
            try_addr
        );

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
        Some(exec_mem as usize),
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
        ((exec_mem as usize).wrapping_add(data_addr_offset as usize))
            as *const u8,
        jump_instruction,
    ))
}

type MessageReplacements =
    Mutex<Vec<(FmgCategories, u32, Arc<Mutex<Vec<u16>>>)>>;

fn create_replacement_message(
    message_replacements: MessageReplacements,
    category: FmgCategories,
    id: u32,
) -> MessageReplacements {
    let replacement_text = r"Yo: We got Dank Memes...
    
    
    ";
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

fn replacement_message_ptr(buffer_box: Box<[u16]>) -> Arc<Mutex<Vec<u16>>> {
    let replacement_msg_ptr = Box::leak(buffer_box);

    Arc::new(Mutex::new(Vec::from(replacement_msg_ptr)))
}

mod test {

    #[test]
    fn test_jump_instructions() {
        let instructions = crate::jump_instructions();
        assert!(
            instructions.is_some(),
            "Jump instructions should be generated"
        );
        let (_offset, buffer) = instructions.unwrap();
        assert_eq!(
            buffer.len(),
            5,
            "Jump instruction buffer should be 5 bytes long"
        );
        dbg!(&buffer[1]);
        assert_eq!(buffer[0], 0xE9, "First byte should be JMP instruction");
    }

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
