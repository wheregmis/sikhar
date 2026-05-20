//! Fluent style utilities inspired by GPUI authoring patterns.

use crate::pixels::{px, Pixels};
use crate::style::{FlexDirection, FontWeight, OverflowAxis, StyleRefinement, TextAlign};
use sparsha_core::Color;

/// Types that expose a mutable [`StyleRefinement`] for fluent chaining.
pub trait Styled {
    fn style_mut(&mut self) -> &mut StyleRefinement;

    fn flex(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display_flex = true;
        self
    }

    fn flex_col(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display_flex = true;
        self.style_mut().flex_direction = Some(FlexDirection::Column);
        self
    }

    fn flex_row(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display_flex = true;
        self.style_mut().flex_direction = Some(FlexDirection::Row);
        self
    }

    fn gap(mut self, value: impl Into<Pixels>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().gap = Some(value.into());
        self
    }

    fn gap_1(mut self) -> Self
    where
        Self: Sized,
    {
        self.gap(px(4.0))
    }

    fn gap_2(mut self) -> Self
    where
        Self: Sized,
    {
        self.gap(px(8.0))
    }

    fn gap_3(mut self) -> Self
    where
        Self: Sized,
    {
        self.gap(px(12.0))
    }

    fn gap_4(mut self) -> Self
    where
        Self: Sized,
    {
        self.gap(px(16.0))
    }

    fn p(mut self, value: impl Into<Pixels>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().padding = Some(value.into());
        self
    }

    fn p_2(mut self) -> Self
    where
        Self: Sized,
    {
        self.p(px(8.0))
    }

    fn p_4(mut self) -> Self
    where
        Self: Sized,
    {
        self.p(px(16.0))
    }

    fn p_6(mut self) -> Self
    where
        Self: Sized,
    {
        self.p(px(24.0))
    }

    fn px_4(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().padding_x = Some(px(16.0));
        self
    }

    fn py_2(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().padding_y = Some(px(8.0));
        self
    }

    fn m_2(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().margin = Some(px(8.0));
        self
    }

    fn size_full(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size_full = true;
        self
    }

    fn w_full(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().fill_width = true;
        self
    }

    fn h_full(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().fill_height = true;
        self
    }

    fn overflow_scroll(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow_x = Some(OverflowAxis::Scroll);
        self.style_mut().overflow_y = Some(OverflowAxis::Scroll);
        self
    }

    fn overflow_y_scroll(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow_y = Some(OverflowAxis::Scroll);
        self
    }

    fn border_1(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().border_width = Some(px(1.0));
        self
    }

    fn border_color(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.style_mut().border_color = Some(color);
        self
    }

    fn rounded_md(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().border_radius = Some(px(6.0));
        self
    }

    fn rounded_lg(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().border_radius = Some(px(8.0));
        self
    }

    fn text_sm(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(14.0));
        self
    }

    fn text_base(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(16.0));
        self
    }

    fn text_lg(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(18.0));
        self
    }

    fn text_xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(20.0));
        self
    }

    fn text_2xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(24.0));
        self
    }

    fn text_4xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_size = Some(px(36.0));
        self
    }

    fn font_medium(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_weight = Some(FontWeight::Medium);
        self
    }

    fn font_semibold(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_weight = Some(FontWeight::Semibold);
        self
    }

    fn font_bold(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().font_weight = Some(FontWeight::Bold);
        self
    }

    fn text_center(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().text_align = Some(TextAlign::Center);
        self
    }

    fn bg(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.style_mut().background = Some(color);
        self
    }

    fn text_color(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.style_mut().text_color = Some(color);
        self
    }

    fn hover(mut self, refine: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        let hover = refine(StyleRefinement::default());
        self.style_mut().hover = Some(Box::new(hover));
        self
    }
}
