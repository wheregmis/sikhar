//! Render trait for element-first components.

use crate::IntoElement;

/// Minimal render context surface implemented by Sparsha's `ComponentContext`.
pub trait ElementRenderContext {
    fn theme_background(&self) -> sparsha_core::Color;
    fn theme_text(&self) -> sparsha_core::Color;
    fn theme_muted_text(&self) -> sparsha_core::Color;
    fn theme_brand(&self) -> sparsha_core::Color;
    fn theme_surface(&self) -> sparsha_core::Color;
    fn theme_border(&self) -> sparsha_core::Color;
}

/// GPUI-inspired render entry for element-first components.
pub trait Render {
    fn render(&mut self, cx: &mut dyn ElementRenderContext) -> impl IntoElement;
}
