#[derive(Default)]
pub struct EguiOptionsState {
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_settings_ui: bool,
    pub show_inspection_ui: bool,
}

pub struct EguiState {
    pub winit_state: egui_winit::State,
    pub context: egui::Context,
    pub render_pass: egui_wgpu::renderer::RenderPass,
    pub options: EguiOptionsState,
}

impl std::fmt::Debug for EguiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EguiState {}")
    }
}
