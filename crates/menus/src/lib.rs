mod fmg;
mod gui;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::gui::HookGui;

use eldenring_util::{program::Program, system::wait_for_system_init};
use fmg::FmgCategories;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use windows::Win32::Foundation::HINSTANCE;

const WILD_STRIKES_ID: u32 = 110;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        let message_replacements: Mutex<Vec<(FmgCategories, u32, Arc<Mutex<Vec<u16>>>)>> =
            Mutex::new(Vec::new());

        let message_replacements = create_replacement_message(
            message_replacements,
            FmgCategories::ArtsCaption,
            WILD_STRIKES_ID,
        );
        let message_replacements = Arc::new(message_replacements);
        std::thread::spawn(move || {
            let program: Program<'_> = Program::current();
            let duration: Duration = Duration::from_millis(5000);
            wait_for_system_init(&program, duration).expect("System initialization timed out");

            let gui = HookGui::new(message_replacements, || {
                hudhook::eject();
            });

            let result = hudhook::Hudhook::builder()
                .with::<ImguiDx12Hooks>(gui)
                .with_hmodule(hmodule)
                .build()
                .apply();

            if let Err(_e) = result {
                hudhook::eject();
            }
        });
    }
}

fn create_replacement_message(
    message_replacements: Mutex<Vec<(FmgCategories, u32, Arc<Mutex<Vec<u16>>>)>>,
    category: FmgCategories,
    id: u32,
) -> Mutex<Vec<(FmgCategories, u32, Arc<Mutex<Vec<u16>>>)>> {
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
    let replacement_msg_ptr = Arc::new(Mutex::new(Vec::from(replacement_msg_ptr)));
    replacement_msg_ptr
}
