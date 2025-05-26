use crate::fmg::FmgCategories;
use hudhook::imgui::{Condition, Context, Ui};
use hudhook::{ImguiRenderLoop, RenderContext};
use pmod::fmg::MsgRepository;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HookGui {
    unhook_triggered: Arc<Mutex<bool>>,
    unhook_fn: Arc<dyn Fn() + Send + Sync>,
    pub window_open: Arc<Mutex<bool>>,
    pub replacement_msg_ptr: Arc<Mutex<Vec<u16>>>,
}

impl HookGui {
    pub fn new(
        replacement_msg_ptr: Arc<Mutex<Vec<u16>>>,
        unhook_fn: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        Self {
            unhook_triggered: Arc::new(Mutex::new(false)),
            unhook_fn: Arc::new(unhook_fn),
            window_open: Arc::new(Mutex::new(false)),
            replacement_msg_ptr,
        }
    }
}

const WILD_STRIKES_ID: u32 = 110;

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
            let gui_size = [800.0, 600.0]; // Default size for the GUI
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

                    MsgRepository::replace_msg(
                        0,
                        FmgCategories::ArtsCaption.into(),
                        WILD_STRIKES_ID,
                        std::ptr::NonNull::new(
                            self.replacement_msg_ptr.lock().unwrap().as_mut_ptr(),
                        ),
                    );
                });
        }
    }
}
