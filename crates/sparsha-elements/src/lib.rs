//! GPUI-inspired element composition API for Sparsha.
//!
//! This crate provides declarative `div().flex().gap_3()` authoring that maps to
//! Tailwind classes on web and Taffy layout on native targets.

mod element;
mod pixels;
mod render;
mod style;
mod styled;
pub mod tailwind_map;
pub mod taffy_map;

pub use element::{
    button, div, text, ButtonElement, Div, ElementHandlers, ElementId, ElementKind, ElementNode,
    IntoElement, ParentElement, TextElement,
};
pub use pixels::{px, Pixels};
pub use render::{ElementRenderContext, Render};
pub use style::{
    FlexDirection, FontWeight, OverflowAxis, StyleRefinement, TextAlign as ElementTextAlign,
};
pub use styled::Styled;
