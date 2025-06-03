use crate::MessageReplacements;
use hudhook::imgui::{Condition, Context, Ui};
use hudhook::{ImguiRenderLoop, RenderContext};
use last_weapon::WeaponData;
use pmod::fmg::MsgRepository;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HookGui {
    unhook_triggered: Arc<Mutex<bool>>,
    unhook_fn: Arc<dyn Fn() + Send + Sync>,
    pub window_open: Arc<Mutex<bool>>,
    pub message_replacements: Arc<MessageReplacements>,
    pub last_weapon_data_ptr: Arc<AtomicUsize>,
}
impl HookGui {
    pub fn new(
        message_replacements: Arc<MessageReplacements>,
        last_weapon_data_ptr: AtomicUsize,
        unhook_fn: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        Self {
            last_weapon_data_ptr: Arc::new(last_weapon_data_ptr),
            unhook_triggered: Arc::new(Mutex::new(false)),
            unhook_fn: Arc::new(unhook_fn),
            window_open: Arc::new(Mutex::new(false)),
            message_replacements,
        }
    }
}

impl ImguiRenderLoop for HookGui {
    fn before_render<'a>(
        &'a mut self,
        ctx: &mut Context,
        _render_context: &'a mut dyn RenderContext,
    ) {
        let open = self.window_open.lock().unwrap();
        let io = ctx.io_mut();
        io.mouse_draw_cursor = *open;
        io.want_capture_mouse = *open;
        io.want_set_mouse_pos = *open;
    }

    fn render(&mut self, ui: &mut Ui) {
        let mut open = self.window_open.lock().unwrap();

        ui.main_menu_bar(|| {
            ui.menu_with_enabled("Hook GUI", true, || {
                if ui.menu_item("Toggle Overlay") {
                    *open = !*open;
                    // The value is already updated in the MutexGuard, no need to lock again
                }
            });
        });

        if *open {
            let gui_size = [100.0, 25.0]; // Default size for the GUI
            ui.window("ER Menus")
                .size(gui_size, Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hook active.");
                    if ui.button("Unhook + Exit") {
                        let mut flag = self.unhook_triggered.lock().unwrap();
                        if !*flag {
                            (self.unhook_fn)();
                            *flag = true;
                        }
                    }

                    ui.separator();
                    let pointer_at_last_weapon_data_ptr = unsafe {
                        std::ptr::read(
                            self.last_weapon_data_ptr.load(Ordering::Relaxed)
                                as *const usize,
                        )
                    };
                    let mut address: [u8; 8] = [0u8; 8];
                    address.copy_from_slice(
                        &pointer_at_last_weapon_data_ptr.to_le_bytes()[..8],
                    );
                    let last_weapon_ptr = usize::from_le_bytes(address);
                    ui.text(format!(
                        "Last Weapon Data Ptr 0x{:X}",
                        last_weapon_ptr
                    ));
                    // Use Serde and WeaponData to read the bytes at the last_weapon_ptr as a WeaponData struct
                    if last_weapon_ptr != 0 {
                        let weapon_data: WeaponData = unsafe {
                            std::ptr::read(last_weapon_ptr as *const WeaponData)
                        };
                        for (category, id, replacement_msg_ptr) in
                            self.message_replacements.lock().unwrap().iter()
                        {
                            let mut message =
                                replacement_msg_ptr.lock().unwrap();
                            let weapon_data_name_to_vec_16 = |name: String| {
                                let mut vec = vec![0u16; 16];
                                for (i, c) in
                                    name.encode_utf16().take(16).enumerate()
                                {
                                    vec[i] = c;
                                }
                                vec
                            };
                            let weapon_data_name =
                                weapon_data_name_to_vec_16(weapon_data.name());
                            // Put the weapon data name at the beginning of the message
                            message.extend(weapon_data_name.clone());

                            MsgRepository::replace_msg(
                                0,
                                *category as u32,
                                *id,
                                std::ptr::NonNull::new(message.as_mut_ptr()),
                            );
                        }

                        ui.text(format!(
                            "Last Weapon Data {}: {:#}",
                            weapon_data.name(),
                            weapon_data
                        ));
                    } else {
                        ui.text("No last weapon data available.");
                    }
                });
        }
    }
}
