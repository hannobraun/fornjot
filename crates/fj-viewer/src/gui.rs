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
pub struct Options {
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_settings_ui: bool,
    pub show_inspection_ui: bool,
}

pub struct Gui {
    pub context: egui::Context,
    pub render_pass: egui_wgpu::renderer::RenderPass,
    pub options: Options,
}

impl Gui {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
    ) -> Self {
        // The implementation of the integration with `egui` is likely to need
        // to change "significantly" depending on what architecture approach is
        // chosen going forward.
        //
        // The current implementation is somewhat complicated by virtue of
        // "sitting somewhere in the middle" in relation to being neither a
        // standalone integration nor fully using `egui` as a framework.
        //
        // This is a result of a combination of the current integration being
        // "proof of concept" level, and using `egui-winit` & `egui-wgpu`, which
        // are both relatively new additions to the core `egui` ecosystem.
        //
        // It is recommended to read the following for additional helpful
        // context for choosing an architecture:
        //
        // - https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/README.md#what-is-the-difference-between-egui-and-eframe
        // - https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/README.md#how-do-i-render-3d-stuff-in-an-egui-area

        let context = egui::Context::default();

        // We need to hold on to this, otherwise it might cause the egui font
        // texture to get dropped after drawing one frame.
        //
        // This then results in an `egui_wgpu_backend` error of
        // `BackendError::Internal` with message:
        //
        // ```
        // Texture 0 used but not live
        // ```
        //
        // See also: <https://github.com/hasenbanck/egui_wgpu_backend/blob/b2d3e7967351690c6425f37cd6d4ffb083a7e8e6/src/lib.rs#L373>
        let render_pass =
            egui_wgpu::renderer::RenderPass::new(device, texture_format, 1);

        Self {
            context,
            render_pass,
            options: Default::default(),
        }
    }
}

impl std::fmt::Debug for Gui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EguiState {}")
    }
}
