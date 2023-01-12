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

use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use std::env::current_dir;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

use fj_math::{Aabb, Scalar};

use crate::{
    graphics::{DrawConfig, DEPTH_FORMAT, SAMPLE_COUNT},
    StatusReport,
};

/// The GUI
pub struct Gui {
    context: egui::Context,
    renderer: egui_wgpu::Renderer,
    options: Options,
    egui_output: Option<egui::FullOutput>,
}

impl Gui {
    pub(crate) fn new(
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
        let renderer = egui_wgpu::Renderer::new(
            device,
            texture_format,
            Some(DEPTH_FORMAT),
            SAMPLE_COUNT,
        );

        Self {
            context,
            renderer,
            options: Options::default(),
            egui_output: None,
        }
    }

    /// Access the egui context
    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub(crate) fn update(
        &mut self,
        pixels_per_point: f32,
        egui_input: egui::RawInput,
        config: &mut DrawConfig,
        aabb: &Aabb<3>,
        line_drawing_available: bool,
        state: GuiState,
    ) -> Option<PathBuf> {
        self.context.set_pixels_per_point(pixels_per_point);
        self.context.begin_frame(egui_input);

        let bounding_box_size = {
            let [x, y, z] = aabb.size().components.map(Scalar::into_f32);
            format!("Model bounding box size:\n{x:0.1} {y:0.1} {z:0.1}")
        };

        egui::SidePanel::left("fj-left-panel").show(&self.context, |ui| {
            ui.add_space(16.0);

            ui.group(|ui| {
                ui.checkbox(&mut config.draw_model, "Render model")
                    .on_hover_text_at_pointer("Toggle with 1");
                ui.add_enabled(line_drawing_available, egui::Checkbox::new(&mut config.draw_mesh, "Render mesh"))
                    .on_hover_text_at_pointer("Toggle with 2")
                    .on_disabled_hover_text(
                        "Rendering device does not have line rendering feature support",
                    );
                ui.add_enabled(line_drawing_available, egui::Checkbox::new(&mut config.draw_debug, "Render debug"))
                    .on_hover_text_at_pointer("Toggle with 3")
                    .on_disabled_hover_text(
                        "Rendering device does not have line rendering feature support"
                    );
                ui.add_space(16.0);
                ui.strong(bounding_box_size);
            });

            ui.add_space(16.0);

            {
                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.options.show_settings_ui,
                        "Show egui settings UI",
                    );
                    if self.options.show_settings_ui {
                        self.context.settings_ui(ui);
                    }
                });

                ui.add_space(16.0);

                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.options.show_inspection_ui,
                        "Show egui inspection UI",
                    );
                    if self.options.show_inspection_ui {
                        ui.indent("indent-inspection-ui", |ui| {
                            self.context.inspection_ui(ui);
                        });
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Originally this was only meant to be a simple demonstration
                // of the `egui` `trace!()` macro...
                //
                // ...but it seems the trace feature can't be enabled
                // separately from the layout debug feature, which all
                // gets a bit messy...
                //
                // ...so, this instead shows one possible way to implement
                // "trace only" style debug text on hover.
                //
                ui.group(|ui| {
                    let label_text = format!(
                        "Show debug text demo.{}",
                        if self.options.show_debug_text_example {
                            " (Hover me.)"
                        } else {
                            ""
                        }
                    );

                    ui.style_mut().wrap = Some(false);

                    if ui
                        .checkbox(
                            &mut self.options.show_debug_text_example,
                            label_text,
                        )
                        .hovered()
                        && self.options.show_debug_text_example
                    {
                        let hover_pos =
                            ui.input().pointer.hover_pos().unwrap_or_default();
                        ui.painter().debug_text(
                            hover_pos,
                            egui::Align2::LEFT_TOP,
                            egui::Color32::DEBUG_COLOR,
                            format!("{:#?}", &config),
                        );
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Demonstration of the `egui` layout debug functionality.
                //
                ui.group(|ui| {
                    //

                    if ui
                        .checkbox(
                            &mut self.options.show_layout_debug_on_hover,
                            "Show layout debug on hover.",
                        )
                        .changed()
                    {
                        ui.ctx().set_debug_on_hover(
                            self.options.show_layout_debug_on_hover,
                        );
                    }

                    ui.scope(|ui| {
                        if self.options.show_trace {
                            egui::trace!(ui, format!("{:?}", &config));
                        }
                    });

                    ui.indent("indent-show-trace", |ui| {
                        ui.set_enabled(
                            self.options.show_layout_debug_on_hover,
                        );

                        ui.checkbox(
                            &mut self.options.show_trace,
                            "Also show egui trace.",
                        );

                        //
                    });
                });
            }

            ui.add_space(16.0);
        });

        egui::Area::new("fj-status-message").show(&self.context, |ui| {
            ui.group(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(format!(
                        "Status:{}",
                        state.status.status()
                    ))
                    .monospace()
                    .color(egui::Color32::BLACK)
                    .background_color(egui::Color32::WHITE),
                ))
            })
        });

        let mut new_model_path = None;

        if !state.model_available {
            egui::Area::new("ask-model")
                .anchor(egui::Align2::CENTER_CENTER, [0_f32, -5_f32])
                .show(&self.context, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(
                            "No model selected please choose a model to view.",
                        ).color(egui::Color32::BLACK)
                        .background_color(egui::Color32::WHITE));
                        if ui
                            .button(egui::RichText::new("Pick a model"))
                            .clicked()
                        {
                            new_model_path = show_file_dialog();
                        }
                    })
                });
        }

        // Even though the output is not used here, `end_frame` must be called
        // at the end of this function. If we don't, and we get into a situation
        // where `update` is called, but `prepare_draw` isn't for a while, the
        // context will keep accumulating output.
        //
        // That might end up being too much output to handle. This can lead to
        // a crash, because a index/vertex buffer gets too full.
        self.egui_output = Some(self.context.end_frame());

        new_model_path
    }

    pub(crate) fn prepare_draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        screen_descriptor: &egui_wgpu::renderer::ScreenDescriptor,
    ) -> Vec<egui::ClippedPrimitive> {
        let Some(egui_output) = self.egui_output.take() else {
            return Vec::new()
        };
        let clipped_primitives = self.context.tessellate(egui_output.shapes);

        for (id, image_delta) in &egui_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }
        for id in &egui_output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        self.renderer.update_buffers(
            device,
            queue,
            encoder,
            &clipped_primitives,
            screen_descriptor,
        );

        clipped_primitives
    }

    pub(crate) fn draw<'s: 'r, 'r>(
        &'s mut self,
        render_pass: &mut wgpu::RenderPass<'r>,
        clipped_primitives: &[egui::ClippedPrimitive],
        screen_descriptor: &egui_wgpu::renderer::ScreenDescriptor,
    ) {
        self.renderer.render(
            render_pass,
            clipped_primitives,
            screen_descriptor,
        );
    }
}

fn show_file_dialog() -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    return FileDialog::new()
        .set_directory(current_dir().unwrap_or_else(|_| PathBuf::from("/")))
        .pick_folder();

    #[cfg(target_arch = "wasm32")]
    todo!("Picking folders does not work on wasm32")
}

impl std::fmt::Debug for Gui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Gui {}")
    }
}

#[derive(Default)]
pub struct Options {
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_settings_ui: bool,
    pub show_inspection_ui: bool,
}

/// The current status of the GUI
pub struct GuiState<'a> {
    /// Reference to the status messages
    pub status: &'a StatusReport,

    /// Indicates whether a model is currently available
    pub model_available: bool,
}
