//! Map [`StyleRefinement`] values to Tailwind utility class strings.

use crate::style::{FlexDirection, FontWeight, OverflowAxis, StyleRefinement, TextAlign};
use sparsha_core::Color;

pub fn classes_from_refinement(refinement: &StyleRefinement) -> Vec<&'static str> {
    let mut classes = Vec::new();

    if refinement.display_flex {
        classes.push("flex");
    }

    match refinement.flex_direction {
        Some(FlexDirection::Column) => classes.push("flex-col"),
        Some(FlexDirection::Row) => classes.push("flex-row"),
        None => {}
    }

    if let Some(grow) = refinement.flex_grow {
        if (grow - 1.0).abs() < f32::EPSILON {
            classes.push("flex-1");
        }
    }

    if let Some(gap) = refinement.gap {
        classes.push(gap_class(gap.value()));
    }

    if let Some(padding) = refinement.padding {
        classes.push(padding_class(padding.value()));
    } else {
        if let Some(px) = refinement.padding_x {
            classes.push(padding_x_class(px.value()));
        }
        if let Some(py) = refinement.padding_y {
            classes.push(padding_y_class(py.value()));
        }
    }

    if refinement.size_full {
        classes.push("w-full");
        classes.push("h-full");
    } else {
        if refinement.fill_width {
            classes.push("w-full");
        }
        if refinement.fill_height {
            classes.push("h-full");
        }
    }

    if matches!(refinement.overflow_x, Some(OverflowAxis::Scroll))
        || matches!(refinement.overflow_y, Some(OverflowAxis::Scroll))
    {
        classes.push("overflow-auto");
    }

    if let Some(width) = refinement.border_width {
        if (width.value() - 1.0).abs() < f32::EPSILON {
            classes.push("border");
        }
    }

    if refinement.border_radius.is_some() {
        classes.push("rounded-md");
    }

    if let Some(size) = refinement.font_size {
        classes.push(text_size_class(size.value()));
    }

    if let Some(weight) = refinement.font_weight {
        classes.push(font_weight_class(weight));
    }

    if let Some(align) = refinement.text_align {
        classes.push(text_align_class(align));
    }

    if refinement.background.is_some() {
        classes.push("bg-surface");
    }

    if refinement.text_color.is_some() {
        classes.push("text-foreground");
    }

    classes
}

pub fn class_string_from_refinement(refinement: &StyleRefinement) -> String {
    let mut parts: Vec<String> = classes_from_refinement(refinement)
        .into_iter()
        .map(str::to_string)
        .collect();
    if let Some(color) = refinement.background {
        parts.push(color_to_tailwind_arbitrary(color, "bg"));
    }
    if let Some(color) = refinement.text_color {
        parts.push(color_to_tailwind_arbitrary(color, "text"));
    }
    if let Some(color) = refinement.border_color {
        parts.push(color_to_tailwind_arbitrary(color, "border"));
    }
    if let Some(hover) = refinement.hover.as_ref() {
        for class in classes_from_refinement(hover) {
            parts.push(format!("hover:{class}"));
        }
    }
    parts.join(" ")
}

pub fn color_to_tailwind_arbitrary(color: Color, prefix: &str) -> String {
    let [r, g, b, a] = color.to_u8_array();
    if (a as f32) < 255.0 {
        format!("{prefix}-[{r} {g} {b} / {:.2}]", a as f32 / 255.0)
    } else {
        format!("{prefix}-[#{r:02x}{g:02x}{b:02x}]")
    }
}

fn gap_class(value: f32) -> &'static str {
    match value as i32 {
        4 => "gap-1",
        8 => "gap-2",
        12 => "gap-3",
        16 => "gap-4",
        24 => "gap-6",
        _ => "gap-3",
    }
}

fn padding_class(value: f32) -> &'static str {
    match value as i32 {
        8 => "p-2",
        16 => "p-4",
        24 => "p-6",
        _ => "p-4",
    }
}

fn padding_x_class(value: f32) -> &'static str {
    match value as i32 {
        16 => "px-4",
        _ => "px-4",
    }
}

fn padding_y_class(value: f32) -> &'static str {
    match value as i32 {
        8 => "py-2",
        _ => "py-2",
    }
}

fn text_size_class(value: f32) -> &'static str {
    match value as i32 {
        14 => "text-sm",
        16 => "text-base",
        18 => "text-lg",
        20 => "text-xl",
        24 => "text-2xl",
        36 => "text-4xl",
        _ => "text-base",
    }
}

fn font_weight_class(weight: FontWeight) -> &'static str {
    match weight {
        FontWeight::Normal => "font-normal",
        FontWeight::Medium => "font-medium",
        FontWeight::Semibold => "font-semibold",
        FontWeight::Bold => "font-bold",
    }
}

fn text_align_class(align: TextAlign) -> &'static str {
    match align {
        TextAlign::Left => "text-left",
        TextAlign::Center => "text-center",
        TextAlign::Right => "text-right",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styled::Styled;
    use crate::Div;
    use sparsha_core::Color;

    #[test]
    fn hello_world_utilities_map_to_tailwind() {
        let div = Div::new()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(Color::from_hex(0x111827));
        let classes = class_string_from_refinement(div.style());
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("gap-3"));
        assert!(classes.contains("p-4"));
    }

    #[test]
    fn scrollable_maps_overflow_class() {
        let div = Div::new().overflow_scroll();
        let classes = class_string_from_refinement(div.style());
        assert!(classes.contains("overflow-auto"));
    }
}
