use sparsha_core::Color;
use sparsha_render::DrawList;

#[cfg(target_arch = "wasm32")]
use crate::web_surface_manager::SurfaceFrame;

pub(crate) struct PlatformDrawFrame<'a> {
    pub(crate) draw_list: &'a DrawList,
    pub(crate) background: Color,
    pub(crate) viewport_width: f32,
    pub(crate) viewport_height: f32,
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub(crate) scale_factor: f32,
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub(crate) elapsed_time: f32,
    #[cfg(target_arch = "wasm32")]
    pub(crate) surface_frames: &'a [SurfaceFrame],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct PlatformDrawOutcome {
    pub(crate) rendered: bool,
    pub(crate) needs_retry: bool,
}

pub(crate) trait PlatformDrawBackend {
    type Error;

    fn render_frame(
        &mut self,
        frame: PlatformDrawFrame<'_>,
    ) -> Result<PlatformDrawOutcome, Self::Error>;
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native {
    use super::{PlatformDrawBackend, PlatformDrawFrame, PlatformDrawOutcome};
    use sparsha_render::Renderer;
    use sparsha_text::TextSystem;

    pub(crate) struct NativeGpuDrawBackend<'a> {
        pub(crate) renderer: &'a mut Renderer,
        pub(crate) device: &'a wgpu::Device,
        pub(crate) queue: &'a wgpu::Queue,
        pub(crate) text_system: &'a mut TextSystem,
        pub(crate) encoder: &'a mut wgpu::CommandEncoder,
        pub(crate) target: &'a wgpu::TextureView,
    }

    impl PlatformDrawBackend for NativeGpuDrawBackend<'_> {
        type Error = core::convert::Infallible;

        fn render_frame(
            &mut self,
            frame: PlatformDrawFrame<'_>,
        ) -> Result<PlatformDrawOutcome, Self::Error> {
            self.renderer.set_viewport(
                frame.viewport_width,
                frame.viewport_height,
                frame.scale_factor,
            );
            self.renderer.set_time(frame.elapsed_time);
            self.renderer
                .prepare(self.device, self.queue, frame.draw_list, self.text_system);
            self.renderer.render(
                self.encoder,
                self.target,
                wgpu::Color {
                    r: frame.background.r as f64,
                    g: frame.background.g as f64,
                    b: frame.background.b as f64,
                    a: frame.background.a as f64,
                },
            );
            Ok(PlatformDrawOutcome {
                rendered: true,
                needs_retry: false,
            })
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) mod web {
    use super::{PlatformDrawBackend, PlatformDrawFrame, PlatformDrawOutcome};
    use crate::dom_renderer::{DomFrameSnapshot, DomRenderer};
    use crate::web_surface_manager::{HybridSurfaceManager, HybridSurfaceStatus};
    use sparsha_core::Color;

    pub(crate) struct WebLayerDrawBackend<'a> {
        pub(crate) dom_renderer: &'a mut DomRenderer,
        pub(crate) surface_manager: &'a mut HybridSurfaceManager,
    }

    impl PlatformDrawBackend for WebLayerDrawBackend<'_> {
        type Error = wasm_bindgen::JsValue;

        fn render_frame(
            &mut self,
            frame: PlatformDrawFrame<'_>,
        ) -> Result<PlatformDrawOutcome, Self::Error> {
            self.dom_renderer.render(&DomFrameSnapshot {
                draw_list: frame.draw_list,
                background: frame.background,
                viewport_width: frame.viewport_width,
                viewport_height: frame.viewport_height,
            })?;

            let surface_outcome = self
                .surface_manager
                .render(frame.surface_frames, Color::TRANSPARENT)?;
            let needs_retry = surface_outcome.needs_retry
                || (!frame.surface_frames.is_empty()
                    && matches!(
                        self.surface_manager.status(),
                        HybridSurfaceStatus::Initializing
                    ));
            Ok(PlatformDrawOutcome {
                rendered: true,
                needs_retry,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sparsha_core::Rect;
    use sparsha_render::{DrawCommand, TextRun};
    use sparsha_text::{TextLayoutAlignment, TextStyle, TextWrap};

    #[derive(Debug, PartialEq)]
    enum ObservedCommand {
        PushClip(Rect),
        Rect(Rect),
        PushTranslation((f32, f32)),
        TextRun {
            text: String,
            position: (f32, f32),
            max_width: Option<f32>,
        },
        PopTranslation,
        PopClip,
    }

    #[derive(Default)]
    struct RecordingDrawBackend {
        observed: Vec<ObservedCommand>,
    }

    impl PlatformDrawBackend for RecordingDrawBackend {
        type Error = core::convert::Infallible;

        fn render_frame(
            &mut self,
            frame: PlatformDrawFrame<'_>,
        ) -> Result<PlatformDrawOutcome, Self::Error> {
            for command in frame.draw_list.commands() {
                match command {
                    DrawCommand::PushClip { bounds } => {
                        self.observed.push(ObservedCommand::PushClip(*bounds))
                    }
                    DrawCommand::Rect { bounds, .. } => {
                        self.observed.push(ObservedCommand::Rect(*bounds))
                    }
                    DrawCommand::PushTranslation { offset } => self
                        .observed
                        .push(ObservedCommand::PushTranslation(*offset)),
                    DrawCommand::TextRun { run } => {
                        self.observed.push(ObservedCommand::TextRun {
                            text: run.text.clone(),
                            position: run.position,
                            max_width: run.max_width,
                        });
                    }
                    DrawCommand::PopTranslation => {
                        self.observed.push(ObservedCommand::PopTranslation)
                    }
                    DrawCommand::PopClip => self.observed.push(ObservedCommand::PopClip),
                    DrawCommand::Line { .. } | DrawCommand::Text { .. } => {}
                }
            }
            Ok(PlatformDrawOutcome {
                rendered: true,
                needs_retry: false,
            })
        }
    }

    fn representative_draw_list() -> DrawList {
        let mut draw_list = DrawList::new();
        draw_list.push_clip(Rect::new(0.0, 0.0, 120.0, 80.0));
        draw_list.rect(Rect::new(4.0, 8.0, 32.0, 16.0), Color::from_hex(0x112233));
        draw_list.push_translation((10.0, 12.0));
        draw_list.push(DrawCommand::TextRun {
            run: TextRun {
                text: "Hello".to_owned(),
                style: TextStyle::default(),
                position: (16.0, 24.0),
                max_width: Some(72.0),
                alignment: TextLayoutAlignment::Center,
                max_lines: Some(1),
                wrap: TextWrap::NoWrap,
            },
        });
        draw_list.pop_translation();
        draw_list.pop_clip();
        draw_list
    }

    #[test]
    fn draw_frame_contract_preserves_clip_translation_and_text_commands() {
        let draw_list = representative_draw_list();
        let mut backend = RecordingDrawBackend::default();
        let outcome = backend
            .render_frame(PlatformDrawFrame {
                draw_list: &draw_list,
                background: Color::WHITE,
                viewport_width: 320.0,
                viewport_height: 240.0,
                scale_factor: 2.0,
                elapsed_time: 1.25,
                #[cfg(target_arch = "wasm32")]
                surface_frames: &[],
            })
            .expect("recording backend is infallible");

        assert_eq!(
            outcome,
            PlatformDrawOutcome {
                rendered: true,
                needs_retry: false,
            }
        );
        assert_eq!(
            backend.observed,
            vec![
                ObservedCommand::PushClip(Rect::new(0.0, 0.0, 120.0, 80.0)),
                ObservedCommand::Rect(Rect::new(4.0, 8.0, 32.0, 16.0)),
                ObservedCommand::PushTranslation((10.0, 12.0)),
                ObservedCommand::TextRun {
                    text: "Hello".to_owned(),
                    position: (16.0, 24.0),
                    max_width: Some(72.0),
                },
                ObservedCommand::PopTranslation,
                ObservedCommand::PopClip,
            ]
        );
    }
}
