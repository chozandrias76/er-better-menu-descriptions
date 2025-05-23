mod fmg;
mod gui;

use crate::gui::HookGui;

use hudhook::hooks::dx12::ImguiDx12Hooks;
use windows::Win32::Foundation::HINSTANCE;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        std::thread::spawn(move || {
            let gui = HookGui::new(|| {
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
