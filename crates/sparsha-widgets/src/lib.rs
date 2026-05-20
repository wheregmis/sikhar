//! Sparsha Widgets - UI widget library.
//!
//! Stability: the supported 1.0 contract is the crate-root widget/theme/context re-export set.

mod accessibility;
mod animation;
mod app_shell;
mod button;
mod checkbox;
mod container;
#[doc(hidden)]
pub mod context;
mod control_state;
mod draw_surface;
mod element_into;
mod element_root;
mod for_each;
mod into_widget;
mod layout_helpers;
mod list;
mod provider;
mod scroll;
mod scroll_model;
mod semantics;
mod text;
mod text_area;
mod text_editor;
mod text_editor_widget;
mod text_input;
mod theme;
mod viewport;
mod widget;

pub use accessibility::{AccessibilityAction, AccessibilityInfo, AccessibilityRole};
pub use animation::{lerp_color, AnimationEasing, ImplicitAnimation, Tween};
pub use app_shell::{AppBar, FloatingActionButton, Scaffold};
#[doc(hidden)]
pub use button::ButtonStyle;
pub use button::{Button, ButtonState, ButtonVariant};
pub use checkbox::Checkbox;
#[doc(hidden)]
pub use checkbox::CheckboxStyle;
pub use container::{Container, CrossAxisAlignment, MainAxisAlignment};
pub use context::{
    BuildContext, EventCommands, EventContext, LayoutContext, PaintCommands, PaintContext,
};
pub use draw_surface::{DrawSurface, DrawSurfaceContext};
pub use element_into::{ButtonWidgetElement, TextInputWidgetElement, TextWidgetElement};
pub use element_root::{element_root, element_to_widget, ElementDomSnapshot, ElementRoot};
pub use for_each::ForEach;
pub use into_widget::IntoWidget;
pub use layout_helpers::{
    Align, Alignment, Center, Expanded, Padding, Positioned, SizedBox, Spacer, Stack,
};
pub use list::{List, ListDirection};
pub use provider::Provider;
#[doc(hidden)]
pub use scroll::ScrollbarStyle;
pub use scroll::{Scroll, ScrollDirection};
pub use semantics::Semantics;
pub use sparsha_text::TextWrap;
pub use text::{Text, TextAlign, TextOverflow, TextVariant};
pub use text_area::TextArea;
#[doc(hidden)]
pub use text_area::TextAreaStyle;
pub use text_editor::TextEditorState;
pub use text_input::TextInput;
#[doc(hidden)]
pub use text_input::TextInputStyle;
pub use theme::{current_theme, set_current_theme, Theme};
#[doc(hidden)]
pub use theme::{ThemeColors, ThemeControls, ThemeRadii, ThemeSpacing, ThemeTypography};
#[doc(hidden)]
pub use viewport::set_current_viewport;
pub use viewport::{current_viewport, ViewportClass, ViewportInfo, ViewportOrientation};
pub(crate) use viewport::{
    responsive_text_area_min_height, responsive_theme_controls, responsive_typography,
};
pub use widget::{Widget, WidgetChildMode};

// Re-export layout types for convenience
pub use sparsha_layout::{styles, taffy, WidgetId};

#[cfg(test)]
mod test_helpers;
