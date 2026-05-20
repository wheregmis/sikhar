//! Declarative style refinements mapped to Tailwind and Taffy.

use crate::pixels::Pixels;
use sparsha_core::Color;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowAxis {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontWeight {
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Accumulated visual and layout refinements for an element node.
#[derive(Clone, Debug, Default)]
pub struct StyleRefinement {
    pub display_flex: bool,
    pub flex_direction: Option<FlexDirection>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub gap: Option<Pixels>,
    pub padding: Option<Pixels>,
    pub padding_x: Option<Pixels>,
    pub padding_y: Option<Pixels>,
    pub margin: Option<Pixels>,
    pub width: Option<Pixels>,
    pub height: Option<Pixels>,
    pub min_width: Option<Pixels>,
    pub min_height: Option<Pixels>,
    pub size_full: bool,
    pub fill_width: bool,
    pub fill_height: bool,
    pub overflow_x: Option<OverflowAxis>,
    pub overflow_y: Option<OverflowAxis>,
    pub background: Option<Color>,
    pub border_width: Option<Pixels>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Pixels>,
    pub font_size: Option<Pixels>,
    pub font_weight: Option<FontWeight>,
    pub text_color: Option<Color>,
    pub text_align: Option<TextAlign>,
    pub hover: Option<Box<StyleRefinement>>,
}

impl StyleRefinement {
    pub fn merge_from(&mut self, other: &StyleRefinement) {
        if other.display_flex {
            self.display_flex = true;
        }
        if other.flex_direction.is_some() {
            self.flex_direction = other.flex_direction;
        }
        if other.flex_grow.is_some() {
            self.flex_grow = other.flex_grow;
        }
        if other.flex_shrink.is_some() {
            self.flex_shrink = other.flex_shrink;
        }
        if other.gap.is_some() {
            self.gap = other.gap;
        }
        if other.padding.is_some() {
            self.padding = other.padding;
        }
        if other.padding_x.is_some() {
            self.padding_x = other.padding_x;
        }
        if other.padding_y.is_some() {
            self.padding_y = other.padding_y;
        }
        if other.margin.is_some() {
            self.margin = other.margin;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.min_width.is_some() {
            self.min_width = other.min_width;
        }
        if other.min_height.is_some() {
            self.min_height = other.min_height;
        }
        if other.size_full {
            self.size_full = true;
        }
        if other.fill_width {
            self.fill_width = true;
        }
        if other.fill_height {
            self.fill_height = true;
        }
        if other.overflow_x.is_some() {
            self.overflow_x = other.overflow_x;
        }
        if other.overflow_y.is_some() {
            self.overflow_y = other.overflow_y;
        }
        if other.background.is_some() {
            self.background = other.background;
        }
        if other.border_width.is_some() {
            self.border_width = other.border_width;
        }
        if other.border_color.is_some() {
            self.border_color = other.border_color;
        }
        if other.border_radius.is_some() {
            self.border_radius = other.border_radius;
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size;
        }
        if other.font_weight.is_some() {
            self.font_weight = other.font_weight;
        }
        if other.text_color.is_some() {
            self.text_color = other.text_color;
        }
        if other.text_align.is_some() {
            self.text_align = other.text_align;
        }
        if let Some(hover) = &other.hover {
            let slot = self.hover.get_or_insert_with(Box::default);
            slot.merge_from(hover);
        }
    }
}
