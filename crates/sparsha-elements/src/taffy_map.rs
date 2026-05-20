//! Map [`StyleRefinement`] values to [`taffy::Style`].

use crate::pixels::Pixels;
use crate::style::{FlexDirection as ElementFlexDirection, OverflowAxis, StyleRefinement};
use taffy::prelude::*;
use taffy::{Overflow, Point};

pub fn style_from_refinement(refinement: &StyleRefinement) -> Style {
    let mut style = Style::default();

    if refinement.display_flex {
        style.display = Display::Flex;
    }

    if let Some(direction) = refinement.flex_direction {
        style.flex_direction = match direction {
            ElementFlexDirection::Row => FlexDirection::Row,
            ElementFlexDirection::Column => FlexDirection::Column,
        };
    }

    if let Some(grow) = refinement.flex_grow {
        style.flex_grow = grow;
    }
    if let Some(shrink) = refinement.flex_shrink {
        style.flex_shrink = shrink;
    }

    if let Some(gap) = refinement.gap {
        let gap = length(gap.value());
        style.gap = Size {
            width: gap,
            height: gap,
        };
    }

    if let Some(all) = refinement.padding {
        style.padding = rect(all);
    } else if refinement.padding_x.is_some() || refinement.padding_y.is_some() {
        style.padding = Rect {
            left: refinement
                .padding_x
                .map(|v| length(v.value()))
                .unwrap_or(length(0.0_f32)),
            right: refinement
                .padding_x
                .map(|v| length(v.value()))
                .unwrap_or(length(0.0_f32)),
            top: refinement
                .padding_y
                .map(|v| length(v.value()))
                .unwrap_or(length(0.0_f32)),
            bottom: refinement
                .padding_y
                .map(|v| length(v.value()))
                .unwrap_or(length(0.0_f32)),
        };
    }

    if refinement.size_full {
        style.size = Size {
            width: percent(1.0_f32),
            height: percent(1.0_f32),
        };
    } else {
        if refinement.fill_width {
            style.size.width = percent(1.0_f32);
        } else if let Some(width) = refinement.width {
            style.size.width = length(width.value());
        }
        if refinement.fill_height {
            style.size.height = percent(1.0_f32);
        } else if let Some(height) = refinement.height {
            style.size.height = length(height.value());
        }
    }

    if let Some(min_width) = refinement.min_width {
        style.min_size.width = length(min_width.value());
    }
    if let Some(min_height) = refinement.min_height {
        style.min_size.height = length(min_height.value());
    }

    style.overflow = Point {
        x: map_overflow(refinement.overflow_x),
        y: map_overflow(refinement.overflow_y),
    };

    style
}

fn rect(value: Pixels) -> Rect<LengthPercentage> {
    let len = length(value.value());
    Rect {
        left: len,
        right: len,
        top: len,
        bottom: len,
    }
}

fn map_overflow(axis: Option<OverflowAxis>) -> Overflow {
    match axis.unwrap_or(OverflowAxis::Visible) {
        OverflowAxis::Visible => Overflow::Visible,
        OverflowAxis::Hidden => Overflow::Hidden,
        OverflowAxis::Scroll => Overflow::Scroll,
        OverflowAxis::Auto => Overflow::Scroll,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixels::px;
    use crate::styled::Styled;
    use crate::Div;

    #[test]
    fn flex_column_gap_maps_to_taffy() {
        let div = Div::new().flex().flex_col().gap_3().p_4();
        let style = style_from_refinement(div.style());
        assert_eq!(style.display, Display::Flex);
        assert_eq!(style.flex_direction, FlexDirection::Column);
        assert_eq!(style.gap.width, length(12.0));
        assert_eq!(style.padding.left, length(16.0));
    }

    #[test]
    fn size_full_maps_to_percent() {
        let div = Div::new().size_full();
        let style = style_from_refinement(div.style());
        assert_eq!(style.size.width, percent(1.0));
        assert_eq!(style.size.height, percent(1.0));
    }

    #[test]
    fn overflow_scroll_maps_to_taffy_scroll() {
        let div = Div::new().overflow_scroll();
        let style = style_from_refinement(div.style());
        assert_eq!(style.overflow.x, Overflow::Scroll);
        assert_eq!(style.overflow.y, Overflow::Scroll);
    }
}
