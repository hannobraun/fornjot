//! GUI-related code
//!
//! If at some point you use `Painter` or similar and you get this error:
//!
//! `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
//!
//! and/or:
//!
//! `wgpu_core::device: surface configuration failed: Native window is in use`
//!
//! it's *probably(?)* because the swap chain has already been created for the
//! window (e.g. by an integration) and *not* because of a regression of this
//! issue (probably):
//!
//! <https://github.com/gfx-rs/wgpu/issues/1492>

#[derive(Default)]
pub struct EguiOptionsState {
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_settings_ui: bool,
    pub show_inspection_ui: bool,
}

pub struct EguiState {
    pub context: egui::Context,
    pub render_pass: egui_wgpu::renderer::RenderPass,
    pub options: EguiOptionsState,
}

impl std::fmt::Debug for EguiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EguiState {}")
    }
}
