//! Theme tokens and runtime theme context.

use sparsha_core::Color;
use std::cell::RefCell;

/// Top-level theme object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Theme {
    pub(crate) colors: ThemeColors,
    pub(crate) typography: ThemeTypography,
    pub(crate) spacing: ThemeSpacing,
    pub(crate) radii: ThemeRadii,
    pub(crate) controls: ThemeControls,
}

impl Theme {
    pub fn light() -> Self {
        Self::default()
    }

    pub fn dark() -> Self {
        Self {
            colors: ThemeColors {
                background: Color::from_hex(0x0F172A),
                surface: Color::from_hex(0x111827),
                surface_variant: Color::from_hex(0x1E293B),
                surface_done: Color::from_hex(0x0B1220),
                text_primary: Color::from_hex(0xE2E8F0),
                text_muted: Color::from_hex(0x94A3B8),
                primary: Color::from_hex(0x3B82F6),
                primary_hovered: Color::from_hex(0x2563EB),
                primary_pressed: Color::from_hex(0x1D4ED8),
                error: Color::from_hex(0xB91C1C),
                error_hovered: Color::from_hex(0x991B1B),
                error_pressed: Color::from_hex(0x7F1D1D),
                border: Color::from_hex(0x334155),
                border_focus: Color::from_hex(0x60A5FA),
                disabled: Color::from_hex(0x64748B),
                input_background: Color::from_hex(0x1E293B),
                input_placeholder: Color::from_hex(0x64748B),
                text_on_primary: Color::WHITE,
            },
            typography: ThemeTypography::default(),
            spacing: ThemeSpacing::default(),
            radii: ThemeRadii::default(),
            controls: ThemeControls::default(),
        }
    }

    pub fn brand(mut self, color: Color) -> Self {
        self.colors.primary = color;
        self.colors.primary_hovered = adjust_color(color, 0.88);
        self.colors.primary_pressed = adjust_color(color, 0.76);
        self.colors.border_focus = adjust_color(color, 1.25);
        self.colors.text_on_primary = readable_text_on(color);
        self
    }

    pub fn brand_states(mut self, primary: Color, hovered: Color, pressed: Color) -> Self {
        self.colors.primary = primary;
        self.colors.primary_hovered = hovered;
        self.colors.primary_pressed = pressed;
        self.colors.border_focus = adjust_color(primary, 1.25);
        self.colors.text_on_primary = readable_text_on(primary);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.colors.background = color;
        self
    }

    pub fn surface(mut self, color: Color) -> Self {
        self.colors.surface = color;
        self.colors.input_background = color;
        self
    }

    pub fn surface_variant(mut self, color: Color) -> Self {
        self.colors.surface_variant = color;
        self
    }

    pub fn surface_done(mut self, color: Color) -> Self {
        self.colors.surface_done = color;
        self
    }

    pub fn text(mut self, color: Color) -> Self {
        self.colors.text_primary = color;
        self
    }

    pub fn muted_text(mut self, color: Color) -> Self {
        self.colors.text_muted = color;
        self.colors.input_placeholder = color;
        self
    }

    pub fn border(mut self, color: Color) -> Self {
        self.colors.border = color;
        self
    }

    pub fn focus(mut self, color: Color) -> Self {
        self.colors.border_focus = color;
        self
    }

    pub fn error(mut self, color: Color) -> Self {
        self.colors.error = color;
        self.colors.error_hovered = adjust_color(color, 0.88);
        self.colors.error_pressed = adjust_color(color, 0.76);
        self
    }

    pub fn error_states(mut self, base: Color, hovered: Color, pressed: Color) -> Self {
        self.colors.error = base;
        self.colors.error_hovered = hovered;
        self.colors.error_pressed = pressed;
        self
    }

    pub fn disabled(mut self, color: Color) -> Self {
        self.colors.disabled = color;
        self
    }

    pub fn input_background(mut self, color: Color) -> Self {
        self.colors.input_background = color;
        self
    }

    pub fn input_placeholder(mut self, color: Color) -> Self {
        self.colors.input_placeholder = color;
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.typography.font_family = family.into();
        self
    }

    pub fn type_scale(mut self, body: f32, small: f32, title: f32, button: f32) -> Self {
        self.typography.body_size = body;
        self.typography.small_size = small;
        self.typography.title_size = title;
        self.typography.button_size = button;
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.typography.line_height = line_height;
        self
    }

    pub fn spacing(mut self, xs: f32, sm: f32, md: f32, lg: f32, xl: f32) -> Self {
        self.spacing = ThemeSpacing { xs, sm, md, lg, xl };
        self
    }

    pub fn radius(mut self, sm: f32, md: f32, lg: f32) -> Self {
        self.radii = ThemeRadii { sm, md, lg };
        self
    }

    pub fn control_size(mut self, height: f32) -> Self {
        self.controls.control_height = height;
        self
    }

    pub fn control_padding(mut self, x: f32, y: f32) -> Self {
        self.controls.control_padding_x = x;
        self.controls.control_padding_y = y;
        self
    }

    pub fn checkbox_size(mut self, size: f32) -> Self {
        self.controls.checkbox_size = size;
        self
    }

    pub fn scrollbar_thickness(mut self, thickness: f32) -> Self {
        self.controls.scrollbar_thickness = thickness;
        self
    }

    pub fn focus_ring_width(mut self, width: f32) -> Self {
        self.controls.focus_ring_width = width;
        self
    }

    pub fn background_color(&self) -> Color {
        self.colors.background
    }

    pub fn surface_color(&self) -> Color {
        self.colors.surface
    }

    pub fn surface_variant_color(&self) -> Color {
        self.colors.surface_variant
    }

    pub fn surface_done_color(&self) -> Color {
        self.colors.surface_done
    }

    pub fn text_color(&self) -> Color {
        self.colors.text_primary
    }

    pub fn muted_text_color(&self) -> Color {
        self.colors.text_muted
    }

    pub fn primary_color(&self) -> Color {
        self.colors.primary
    }

    pub fn primary_hovered_color(&self) -> Color {
        self.colors.primary_hovered
    }

    pub fn primary_pressed_color(&self) -> Color {
        self.colors.primary_pressed
    }

    pub fn text_on_primary_color(&self) -> Color {
        self.colors.text_on_primary
    }

    pub fn error_color(&self) -> Color {
        self.colors.error
    }

    pub fn error_hovered_color(&self) -> Color {
        self.colors.error_hovered
    }

    pub fn error_pressed_color(&self) -> Color {
        self.colors.error_pressed
    }

    pub fn border_color(&self) -> Color {
        self.colors.border
    }

    pub fn focus_color(&self) -> Color {
        self.colors.border_focus
    }

    pub fn disabled_color(&self) -> Color {
        self.colors.disabled
    }

    pub fn input_background_color(&self) -> Color {
        self.colors.input_background
    }

    pub fn input_placeholder_color(&self) -> Color {
        self.colors.input_placeholder
    }

    pub fn font_family_name(&self) -> &str {
        &self.typography.font_family
    }

    pub fn body_size(&self) -> f32 {
        self.typography.body_size
    }

    pub fn small_size(&self) -> f32 {
        self.typography.small_size
    }

    pub fn title_size(&self) -> f32 {
        self.typography.title_size
    }

    pub fn button_size(&self) -> f32 {
        self.typography.button_size
    }

    pub fn line_height_value(&self) -> f32 {
        self.typography.line_height
    }

    pub fn spacing_xs(&self) -> f32 {
        self.spacing.xs
    }

    pub fn spacing_sm(&self) -> f32 {
        self.spacing.sm
    }

    pub fn spacing_md(&self) -> f32 {
        self.spacing.md
    }

    pub fn spacing_lg(&self) -> f32 {
        self.spacing.lg
    }

    pub fn spacing_xl(&self) -> f32 {
        self.spacing.xl
    }

    pub fn radius_sm(&self) -> f32 {
        self.radii.sm
    }

    pub fn radius_md(&self) -> f32 {
        self.radii.md
    }

    pub fn radius_lg(&self) -> f32 {
        self.radii.lg
    }

    pub fn control_height(&self) -> f32 {
        self.controls.control_height
    }

    pub fn control_padding_x(&self) -> f32 {
        self.controls.control_padding_x
    }

    pub fn control_padding_y(&self) -> f32 {
        self.controls.control_padding_y
    }

    pub fn focus_ring_width_value(&self) -> f32 {
        self.controls.focus_ring_width
    }

    pub fn checkbox_size_value(&self) -> f32 {
        self.controls.checkbox_size
    }

    pub fn scrollbar_thickness_value(&self) -> f32 {
        self.controls.scrollbar_thickness
    }
}

/// Color tokens used by core widgets.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeColors {
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub surface_done: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub primary: Color,
    pub primary_hovered: Color,
    pub primary_pressed: Color,
    pub error: Color,
    pub error_hovered: Color,
    pub error_pressed: Color,
    pub border: Color,
    pub border_focus: Color,
    pub disabled: Color,
    pub input_background: Color,
    pub input_placeholder: Color,
    pub text_on_primary: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: Color::from_hex(0xF3F4F6),
            surface: Color::WHITE,
            surface_variant: Color::from_hex(0xF1F5F9),
            surface_done: Color::from_hex(0xE2E8F0),
            text_primary: Color::from_hex(0x1F2937),
            text_muted: Color::from_hex(0x6B7280),
            primary: Color::from_hex(0x3B82F6),
            primary_hovered: Color::from_hex(0x2563EB),
            primary_pressed: Color::from_hex(0x1D4ED8),
            error: Color::from_hex(0xDC2626),
            error_hovered: Color::from_hex(0xB91C1C),
            error_pressed: Color::from_hex(0x991B1B),
            border: Color::from_hex(0xD1D5DB),
            border_focus: Color::from_hex(0x60A5FA),
            disabled: Color::from_hex(0x9CA3AF),
            input_background: Color::WHITE,
            input_placeholder: Color::from_hex(0x9CA3AF),
            text_on_primary: Color::WHITE,
        }
    }
}

/// Typography tokens used by core widgets.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeTypography {
    pub font_family: String,
    pub body_size: f32,
    pub small_size: f32,
    pub title_size: f32,
    pub button_size: f32,
    pub line_height: f32,
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            font_family: String::from("Inter"),
            body_size: 16.0,
            small_size: 12.0,
            title_size: 24.0,
            button_size: 14.0,
            line_height: 1.2,
        }
    }
}

/// Spacing tokens used by core widgets.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeSpacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
        }
    }
}

/// Radius tokens used by core widgets.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeRadii {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for ThemeRadii {
    fn default() -> Self {
        Self {
            sm: 4.0,
            md: 6.0,
            lg: 12.0,
        }
    }
}

/// Shared control metrics used by the built-in widgets.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeControls {
    pub control_height: f32,
    pub control_padding_x: f32,
    pub control_padding_y: f32,
    pub focus_ring_width: f32,
    pub checkbox_size: f32,
    pub scrollbar_thickness: f32,
}

impl Default for ThemeControls {
    fn default() -> Self {
        Self {
            control_height: 38.0,
            control_padding_x: 12.0,
            control_padding_y: 8.0,
            focus_ring_width: 2.0,
            checkbox_size: 18.0,
            scrollbar_thickness: 10.0,
        }
    }
}

thread_local! {
    static CURRENT_THEME: RefCell<Theme> = RefCell::new(Theme::default());
}

/// Set the current app theme for this thread.
pub fn set_current_theme(theme: Theme) {
    CURRENT_THEME.with(|slot| {
        *slot.borrow_mut() = theme;
    });
}

/// Read the current app theme.
pub fn current_theme() -> Theme {
    CURRENT_THEME.with(|slot| slot.borrow().clone())
}

fn adjust_color(color: Color, factor: f32) -> Color {
    Color::rgba(
        (color.r * factor).clamp(0.0, 1.0),
        (color.g * factor).clamp(0.0, 1.0),
        (color.b * factor).clamp(0.0, 1.0),
        color.a,
    )
}

fn readable_text_on(color: Color) -> Color {
    let luminance = 0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b;
    if luminance > 0.45 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}
