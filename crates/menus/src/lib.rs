mod fmg;
mod gui;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::gui::HookGui;

use eldenring_util::{program::Program, system::wait_for_system_init};
use hudhook::hooks::dx12::ImguiDx12Hooks;
use windows::Win32::Foundation::HINSTANCE;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        let replacement_msg_ptr = create_replacement_message();
        std::thread::spawn(move || {
            let program: Program<'_> = Program::current();
            let duration: Duration = Duration::from_millis(5000);
            wait_for_system_init(&program, duration).expect("System initialization timed out");

            let gui = HookGui::new(replacement_msg_ptr, || {
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

fn create_replacement_message() -> Arc<Mutex<Vec<u16>>> {
    let replacement_text = r"Yo: We got Dank Memes

Spin around to get dizzy.
Hold down to get funny.";
    let buffer: Vec<u16> = replacement_text.encode_utf16().collect();
    let buffer_box = buffer.into_boxed_slice();
    replacement_message(buffer_box)
}

fn replacement_message(buffer_box: Box<[u16]>) -> Arc<Mutex<Vec<u16>>> {
    let replacement_msg_ptr = Box::leak(buffer_box);
    let replacement_msg_ptr = Arc::new(Mutex::new(Vec::from(replacement_msg_ptr)));
    replacement_msg_ptr
}
