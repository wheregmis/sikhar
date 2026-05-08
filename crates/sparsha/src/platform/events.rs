use crate::platform::{PlatformId, PlatformId::Web};
use sparsha_input::{
    shortcuts, Action, ActionMapper, InputEvent, Key, KeyboardEvent, Modifiers, NamedKey,
    PointerButton, ShortcutProfile, StandardAction,
};

#[cfg(not(target_arch = "wasm32"))]
use winit::event::{ElementState, Ime, MouseButton};

pub(crate) trait PlatformEventTranslator {
    fn platform_id(&self) -> PlatformId;
    fn shortcut_policy(&self) -> PlatformShortcutPolicy {
        PlatformShortcutPolicy::for_platform(self.platform_id())
    }
}

#[allow(unused_imports)]
pub(crate) mod native {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use super::NativeEventTranslator as EventTranslator;
}

#[allow(unused_imports)]
pub(crate) mod web {
    pub(crate) use super::WebEventTranslator as EventTranslator;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PlatformShortcutPolicy {
    profile: ShortcutProfile,
}

impl PlatformShortcutPolicy {
    pub(crate) const fn for_platform(platform_id: PlatformId) -> Self {
        Self {
            profile: platform_id.shortcut_profile(),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn shortcut_profile(self) -> ShortcutProfile {
        self.profile
    }

    #[allow(dead_code)]
    pub(crate) const fn primary_modifiers(self) -> Modifiers {
        self.profile.primary_modifiers()
    }

    pub(crate) fn primary_modifier_active(self, modifiers: Modifiers) -> bool {
        shortcuts::primary_modifier_for(self.profile, modifiers)
    }

    fn action_mapper(self) -> ActionMapper {
        ActionMapper::with_shortcut_profile(self.profile)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct NativeKeyboardDispatch {
    pub(crate) keyboard_event: Option<InputEvent>,
    pub(crate) text_event: Option<InputEvent>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NativeEventTranslator {
    platform_id: PlatformId,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeEventTranslator {
    pub(crate) const fn new(platform_id: PlatformId) -> Self {
        Self { platform_id }
    }

    pub(crate) fn cursor_position(&self, x: f32, y: f32, scale_factor: f32) -> glam::Vec2 {
        glam::Vec2::new(x / scale_factor.max(1.0), y / scale_factor.max(1.0))
    }

    pub(crate) fn map_mouse_button(&self, button: MouseButton) -> PointerButton {
        match button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Auxiliary,
            _ => PointerButton::Primary,
        }
    }

    pub(crate) fn translate_cursor_moved(&self, x: f32, y: f32, scale_factor: f32) -> InputEvent {
        self.translate_pointer_move(self.cursor_position(x, y, scale_factor))
    }

    pub(crate) fn translate_pointer_move(&self, pos: glam::Vec2) -> InputEvent {
        InputEvent::PointerMove { pos }
    }

    pub(crate) fn translate_mouse_input(
        &self,
        pos: glam::Vec2,
        button: MouseButton,
        state: ElementState,
    ) -> InputEvent {
        let button = self.map_mouse_button(button);
        if state.is_pressed() {
            InputEvent::PointerDown { pos, button }
        } else {
            InputEvent::PointerUp { pos, button }
        }
    }

    pub(crate) fn translate_mouse_wheel(
        &self,
        pos: glam::Vec2,
        delta: winit::event::MouseScrollDelta,
        modifiers: Modifiers,
    ) -> InputEvent {
        let delta = match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => glam::Vec2::new(x, y),
            winit::event::MouseScrollDelta::PixelDelta(p) => {
                glam::Vec2::new(p.x as f32 / 20.0, p.y as f32 / 20.0)
            }
        };
        InputEvent::Scroll {
            pos,
            delta,
            modifiers,
        }
    }

    pub(crate) fn translate_focus(&self, focused: bool) -> InputEvent {
        if focused {
            InputEvent::FocusGained
        } else {
            InputEvent::FocusLost
        }
    }

    pub(crate) fn map_modifiers(&self, modifiers: winit::keyboard::ModifiersState) -> Modifiers {
        let mut converted = Modifiers::empty();
        if modifiers.shift_key() {
            converted |= Modifiers::SHIFT;
        }
        if modifiers.control_key() {
            converted |= Modifiers::CONTROL;
        }
        if modifiers.alt_key() {
            converted |= Modifiers::ALT;
        }
        if modifiers.super_key() {
            converted |= Modifiers::META;
        }
        converted
    }

    pub(crate) fn map_key(&self, key: &winit::keyboard::Key<&str>) -> Option<Key> {
        Some(match key {
            winit::keyboard::Key::Character(value) => Key::Character(value.to_string()),
            winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space) => {
                Key::Character(" ".to_owned())
            }
            winit::keyboard::Key::Named(named) => Key::Named(match named {
                winit::keyboard::NamedKey::Enter => NamedKey::Enter,
                winit::keyboard::NamedKey::Tab => NamedKey::Tab,
                winit::keyboard::NamedKey::Backspace => NamedKey::Backspace,
                winit::keyboard::NamedKey::Delete => NamedKey::Delete,
                winit::keyboard::NamedKey::Escape => NamedKey::Escape,
                winit::keyboard::NamedKey::ArrowUp => NamedKey::ArrowUp,
                winit::keyboard::NamedKey::ArrowDown => NamedKey::ArrowDown,
                winit::keyboard::NamedKey::ArrowLeft => NamedKey::ArrowLeft,
                winit::keyboard::NamedKey::ArrowRight => NamedKey::ArrowRight,
                winit::keyboard::NamedKey::Home => NamedKey::Home,
                winit::keyboard::NamedKey::End => NamedKey::End,
                winit::keyboard::NamedKey::PageUp => NamedKey::PageUp,
                winit::keyboard::NamedKey::PageDown => NamedKey::PageDown,
                _ => return None,
            }),
            _ => return None,
        })
    }

    pub(crate) fn should_emit_text(&self, text: &str, modifiers: Modifiers) -> bool {
        !text.is_empty()
            && text.chars().all(|ch| !ch.is_control())
            && !self.shortcut_policy().primary_modifier_active(modifiers)
            && !modifiers.alt()
    }

    pub(crate) fn translate_keyboard(
        &self,
        key: &winit::keyboard::Key<&str>,
        state: ElementState,
        modifiers: Modifiers,
        repeat: bool,
        text: Option<&str>,
    ) -> NativeKeyboardDispatch {
        use sparsha_input::ui_events::keyboard::Code;

        let keyboard_event = self.map_key(key).map(|key| {
            let mut keyboard_event = if state.is_pressed() {
                KeyboardEvent::key_down(key, Code::Unidentified)
            } else {
                KeyboardEvent::key_up(key, Code::Unidentified)
            };
            keyboard_event.modifiers = modifiers;
            keyboard_event.repeat = repeat;
            if state.is_pressed() {
                InputEvent::KeyDown {
                    event: keyboard_event,
                }
            } else {
                InputEvent::KeyUp {
                    event: keyboard_event,
                }
            }
        });

        let text_event = if state.is_pressed() && !repeat {
            text.filter(|value| self.should_emit_text(value, modifiers))
                .map(|value| InputEvent::TextInput {
                    text: value.to_owned(),
                })
        } else {
            None
        };

        NativeKeyboardDispatch {
            keyboard_event,
            text_event,
        }
    }

    pub(crate) fn translate_ime(&self, event: &Ime, ime_composing: bool) -> Vec<InputEvent> {
        match event {
            Ime::Enabled | Ime::Disabled => Vec::new(),
            Ime::Preedit(text, _) => {
                let mut events = Vec::new();
                if !ime_composing {
                    events.push(InputEvent::CompositionStart);
                }
                events.push(InputEvent::CompositionUpdate { text: text.clone() });
                events
            }
            Ime::Commit(text) => vec![InputEvent::CompositionEnd { text: text.clone() }],
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl PlatformEventTranslator for NativeEventTranslator {
    fn platform_id(&self) -> PlatformId {
        self.platform_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct WebKeyboardDispatch {
    pub(crate) keyboard_event: Option<InputEvent>,
    pub(crate) text_event: Option<InputEvent>,
    pub(crate) prevent_default: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WebEventTranslator;

#[allow(dead_code)]
impl WebEventTranslator {
    pub(crate) const fn new() -> Self {
        Self
    }

    pub(crate) fn modifiers_from_flags(
        &self,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    ) -> Modifiers {
        let mut modifiers = Modifiers::empty();
        if shift {
            modifiers |= Modifiers::SHIFT;
        }
        if ctrl {
            modifiers |= Modifiers::CONTROL;
        }
        if alt {
            modifiers |= Modifiers::ALT;
        }
        if meta {
            modifiers |= Modifiers::META;
        }
        modifiers
    }

    pub(crate) fn map_key(&self, key: &str) -> Option<Key> {
        Some(match key {
            "Enter" => Key::Named(NamedKey::Enter),
            "Tab" => Key::Named(NamedKey::Tab),
            "Backspace" => Key::Named(NamedKey::Backspace),
            "Delete" => Key::Named(NamedKey::Delete),
            "Escape" => Key::Named(NamedKey::Escape),
            "ArrowUp" => Key::Named(NamedKey::ArrowUp),
            "ArrowDown" => Key::Named(NamedKey::ArrowDown),
            "ArrowLeft" => Key::Named(NamedKey::ArrowLeft),
            "ArrowRight" => Key::Named(NamedKey::ArrowRight),
            "Home" => Key::Named(NamedKey::Home),
            "End" => Key::Named(NamedKey::End),
            "PageUp" => Key::Named(NamedKey::PageUp),
            "PageDown" => Key::Named(NamedKey::PageDown),
            value if value.chars().count() == 1 => Key::Character(value.to_owned()),
            _ => return None,
        })
    }

    pub(crate) fn map_mouse_button(&self, button: i16) -> PointerButton {
        match button {
            0 => PointerButton::Primary,
            1 => PointerButton::Auxiliary,
            2 => PointerButton::Secondary,
            _ => PointerButton::Primary,
        }
    }

    pub(crate) fn translate_pointer_move(&self, pos: glam::Vec2) -> InputEvent {
        InputEvent::PointerMove { pos }
    }

    pub(crate) fn translate_pointer_down(&self, pos: glam::Vec2, button: i16) -> InputEvent {
        InputEvent::PointerDown {
            pos,
            button: self.map_mouse_button(button),
        }
    }

    pub(crate) fn translate_pointer_up(&self, pos: glam::Vec2, button: i16) -> InputEvent {
        InputEvent::PointerUp {
            pos,
            button: self.map_mouse_button(button),
        }
    }

    pub(crate) fn translate_primary_pointer_down(&self, pos: glam::Vec2) -> InputEvent {
        InputEvent::PointerDown {
            pos,
            button: PointerButton::Primary,
        }
    }

    pub(crate) fn translate_primary_pointer_up(&self, pos: glam::Vec2) -> InputEvent {
        InputEvent::PointerUp {
            pos,
            button: PointerButton::Primary,
        }
    }

    pub(crate) fn translate_scroll(
        &self,
        pos: glam::Vec2,
        delta: glam::Vec2,
        modifiers: Modifiers,
    ) -> InputEvent {
        InputEvent::Scroll {
            pos,
            delta,
            modifiers,
        }
    }

    pub(crate) fn translate_focus(&self, focused: bool) -> InputEvent {
        if focused {
            InputEvent::FocusGained
        } else {
            InputEvent::FocusLost
        }
    }

    fn is_plain_printable_key(&self, key: &str, ctrl: bool, alt: bool, meta: bool) -> bool {
        key.chars().count() == 1 && !ctrl && !alt && !meta
    }

    pub(crate) fn should_emit_text(&self, key: &str, ctrl: bool, alt: bool, meta: bool) -> bool {
        self.is_plain_printable_key(key, ctrl, alt, meta)
    }

    pub(crate) fn should_forward_keydown_to_widget_tree(
        &self,
        focused_text_editor: bool,
        key: &str,
        ctrl: bool,
        alt: bool,
        meta: bool,
    ) -> bool {
        !focused_text_editor || !self.is_plain_printable_key(key, ctrl, alt, meta)
    }

    pub(crate) fn should_prevent_keydown_for_text_editor_action(
        &self,
        event: &KeyboardEvent,
        action: StandardAction,
    ) -> bool {
        matches!(
            action,
            StandardAction::FocusNext
                | StandardAction::FocusPrevious
                | StandardAction::SelectAll
                | StandardAction::Undo
                | StandardAction::Redo
                | StandardAction::Backspace
                | StandardAction::Delete
                | StandardAction::MoveLeft
                | StandardAction::MoveRight
                | StandardAction::MoveUp
                | StandardAction::MoveDown
                | StandardAction::MoveWordLeft
                | StandardAction::MoveWordRight
                | StandardAction::MoveToStart
                | StandardAction::MoveToEnd
                | StandardAction::SelectLeft
                | StandardAction::SelectRight
                | StandardAction::SelectUp
                | StandardAction::SelectDown
                | StandardAction::SelectWordLeft
                | StandardAction::SelectWordRight
                | StandardAction::SelectToStart
                | StandardAction::SelectToEnd
                | StandardAction::Cancel
        ) || matches!(action, StandardAction::Activate)
            && !matches!(&event.key, Key::Character(value) if value == " ")
    }

    pub(crate) fn translate_key_down(
        &self,
        key: &str,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
        focused_text_editor: bool,
    ) -> WebKeyboardDispatch {
        use sparsha_input::ui_events::keyboard::Code;

        let modifiers = self.modifiers_from_flags(shift, ctrl, alt, meta);
        let keyboard_event = self.map_key(key).map(|key| {
            let mut event = KeyboardEvent::key_down(key, Code::Unidentified);
            event.modifiers = modifiers;
            event
        });

        let mapped_action = keyboard_event.as_ref().and_then(|event| {
            self.shortcut_policy()
                .action_mapper()
                .map_event(&InputEvent::KeyDown {
                    event: event.clone(),
                })
        });

        let prevent_default = if focused_text_editor {
            mapped_action
                .and_then(|action| match action {
                    Action::Standard(action) => Some(action),
                    _ => None,
                })
                .map(|action| {
                    self.should_prevent_keydown_for_text_editor_action(
                        keyboard_event.as_ref().expect("keyboard event present"),
                        action,
                    )
                })
                .unwrap_or(false)
        } else {
            matches!(
                mapped_action,
                Some(Action::Standard(
                    StandardAction::FocusNext | StandardAction::FocusPrevious
                ))
            )
        };

        WebKeyboardDispatch {
            keyboard_event: keyboard_event.and_then(|event| {
                self.should_forward_keydown_to_widget_tree(
                    focused_text_editor,
                    key,
                    ctrl,
                    alt,
                    meta,
                )
                .then_some(InputEvent::KeyDown { event })
            }),
            text_event: (!focused_text_editor && self.should_emit_text(key, ctrl, alt, meta)).then(
                || InputEvent::TextInput {
                    text: key.to_owned(),
                },
            ),
            prevent_default,
        }
    }

    pub(crate) fn translate_key_up(
        &self,
        key: &str,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    ) -> Option<InputEvent> {
        use sparsha_input::ui_events::keyboard::Code;

        let mapped = self.map_key(key)?;
        let mut event = KeyboardEvent::key_up(mapped, Code::Unidentified);
        event.modifiers = self.modifiers_from_flags(shift, ctrl, alt, meta);
        Some(InputEvent::KeyUp { event })
    }

    pub(crate) fn translate_before_input(
        &self,
        input_type: &str,
        data: Option<String>,
        bridge_syncing: bool,
        focused_text_editor: bool,
    ) -> Option<InputEvent> {
        if bridge_syncing || !focused_text_editor {
            return None;
        }
        match input_type {
            "insertText" => data.map(|text| InputEvent::TextInput { text }),
            _ => None,
        }
    }

    pub(crate) fn translate_composition_start(
        &self,
        bridge_syncing: bool,
        focused_text_editor: bool,
    ) -> Option<InputEvent> {
        (!bridge_syncing && focused_text_editor).then_some(InputEvent::CompositionStart)
    }

    pub(crate) fn translate_composition_update(
        &self,
        text: String,
        bridge_syncing: bool,
        focused_text_editor: bool,
    ) -> Option<InputEvent> {
        (!bridge_syncing && focused_text_editor).then_some(InputEvent::CompositionUpdate { text })
    }

    pub(crate) fn translate_composition_end(
        &self,
        text: String,
        bridge_syncing: bool,
        focused_text_editor: bool,
    ) -> Option<InputEvent> {
        (!bridge_syncing && focused_text_editor).then_some(InputEvent::CompositionEnd { text })
    }

    pub(crate) fn translate_paste(
        &self,
        text: Option<String>,
        focused_text_editor: bool,
    ) -> Option<InputEvent> {
        focused_text_editor
            .then_some(text)
            .flatten()
            .map(|text| InputEvent::Paste { text })
    }
}

impl PlatformEventTranslator for WebEventTranslator {
    fn platform_id(&self) -> PlatformId {
        Web
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sparsha_input::ui_events::keyboard::Code;

    #[test]
    fn web_shortcut_dispatch_uses_explicit_command_profile() {
        let translator = WebEventTranslator::new();
        let dispatch = translator.translate_key_down("c", false, false, false, true, true);
        assert!(!dispatch.prevent_default);
        assert!(matches!(
            dispatch.keyboard_event,
            Some(InputEvent::KeyDown { .. })
        ));
    }

    #[test]
    fn web_tab_focus_navigation_prevents_browser_default_outside_text_editors() {
        let translator = WebEventTranslator::new();
        let dispatch = translator.translate_key_down("Tab", false, false, false, false, false);
        assert!(dispatch.prevent_default);
    }

    #[test]
    fn web_plain_printable_keys_stay_in_browser_for_text_editors() {
        let translator = WebEventTranslator::new();
        let dispatch = translator.translate_key_down("h", false, false, false, false, true);
        assert!(dispatch.keyboard_event.is_none());
        assert!(dispatch.text_event.is_none());
    }

    #[test]
    fn web_navigation_keys_still_flow_to_text_editors() {
        let translator = WebEventTranslator::new();
        let dispatch = translator.translate_key_down("ArrowLeft", false, false, false, false, true);
        assert!(matches!(
            dispatch.keyboard_event,
            Some(InputEvent::KeyDown { .. })
        ));
        assert!(dispatch.prevent_default);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_text_emission_respects_shortcut_profile() {
        let mac = NativeEventTranslator::new(PlatformId::MacOs);
        assert!(!mac.should_emit_text("c", Modifiers::META));

        let linux = NativeEventTranslator::new(PlatformId::Linux);
        assert!(!linux.should_emit_text("c", Modifiers::CONTROL));
        assert!(linux.should_emit_text("c", Modifiers::SHIFT));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_keyboard_translation_normalizes_space() {
        let translator = NativeEventTranslator::new(PlatformId::Linux);
        let dispatch = translator.translate_keyboard(
            &winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space),
            ElementState::Pressed,
            Modifiers::empty(),
            false,
            Some(" "),
        );
        assert!(matches!(
            dispatch.keyboard_event,
            Some(InputEvent::KeyDown {
                event: KeyboardEvent {
                    key: Key::Character(ref value),
                    code: Code::Unidentified,
                    ..
                },
            }) if value == " "
        ));
        assert!(matches!(
            dispatch.text_event,
            Some(InputEvent::TextInput { text }) if text == " "
        ));
    }

    #[test]
    fn shortcut_policy_makes_primary_modifier_explicit() {
        let web = PlatformShortcutPolicy::for_platform(PlatformId::Web);
        assert_eq!(web.shortcut_profile(), ShortcutProfile::CommandPrimary);
        assert_eq!(web.primary_modifiers(), Modifiers::META);
        assert!(web.primary_modifier_active(Modifiers::META));
        assert!(!web.primary_modifier_active(Modifiers::CONTROL));

        let linux = PlatformShortcutPolicy::for_platform(PlatformId::Linux);
        assert_eq!(linux.shortcut_profile(), ShortcutProfile::ControlPrimary);
        assert_eq!(linux.primary_modifiers(), Modifiers::CONTROL);
        assert!(linux.primary_modifier_active(Modifiers::CONTROL));
        assert!(!linux.primary_modifier_active(Modifiers::META));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_and_web_adapters_share_pointer_scroll_and_focus_semantics() {
        let native = NativeEventTranslator::new(PlatformId::Linux);
        let web = WebEventTranslator::new();
        let pos = glam::Vec2::new(12.0, 24.0);

        assert!(matches!(
            native.translate_pointer_move(pos),
            InputEvent::PointerMove { pos: observed } if observed == pos
        ));
        assert!(matches!(
            web.translate_pointer_move(pos),
            InputEvent::PointerMove { pos: observed } if observed == pos
        ));
        assert!(matches!(
            native.translate_mouse_input(pos, MouseButton::Left, ElementState::Pressed),
            InputEvent::PointerDown {
                pos: observed,
                button: PointerButton::Primary,
            } if observed == pos
        ));
        assert!(matches!(
            web.translate_pointer_down(pos, 0),
            InputEvent::PointerDown {
                pos: observed,
                button: PointerButton::Primary,
            } if observed == pos
        ));

        let modifiers = Modifiers::SHIFT;
        assert!(matches!(
            native.translate_mouse_wheel(
                pos,
                winit::event::MouseScrollDelta::LineDelta(1.0, -2.0),
                modifiers,
            ),
            InputEvent::Scroll {
                pos: observed_pos,
                delta,
                modifiers: observed_modifiers,
            } if observed_pos == pos
                && delta == glam::Vec2::new(1.0, -2.0)
                && observed_modifiers == modifiers
        ));
        assert!(matches!(
            web.translate_scroll(pos, glam::Vec2::new(1.0, -2.0), modifiers),
            InputEvent::Scroll {
                pos: observed_pos,
                delta,
                modifiers: observed_modifiers,
            } if observed_pos == pos
                && delta == glam::Vec2::new(1.0, -2.0)
                && observed_modifiers == modifiers
        ));

        assert!(matches!(
            native.translate_focus(true),
            InputEvent::FocusGained
        ));
        assert!(matches!(web.translate_focus(true), InputEvent::FocusGained));
        assert!(matches!(
            native.translate_focus(false),
            InputEvent::FocusLost
        ));
        assert!(matches!(web.translate_focus(false), InputEvent::FocusLost));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_and_web_adapters_share_keyboard_composition_and_clipboard_semantics() {
        let native = NativeEventTranslator::new(PlatformId::MacOs);
        let web = WebEventTranslator::new();

        let native_key = native.translate_keyboard(
            &winit::keyboard::Key::Character("a"),
            ElementState::Pressed,
            Modifiers::empty(),
            false,
            Some("a"),
        );
        let web_key = web.translate_key_down("a", false, false, false, false, false);
        assert!(matches!(
            native_key.keyboard_event,
            Some(InputEvent::KeyDown { .. })
        ));
        assert!(matches!(
            web_key.keyboard_event,
            Some(InputEvent::KeyDown { .. })
        ));
        assert!(matches!(
            native_key.text_event,
            Some(InputEvent::TextInput { text }) if text == "a"
        ));
        assert!(matches!(
            web_key.text_event,
            Some(InputEvent::TextInput { text }) if text == "a"
        ));

        let native_composition =
            native.translate_ime(&winit::event::Ime::Preedit("ka".to_owned(), None), false);
        assert_eq!(native_composition.len(), 2);
        assert!(matches!(
            native_composition[0],
            InputEvent::CompositionStart
        ));
        assert!(matches!(
            &native_composition[1],
            InputEvent::CompositionUpdate { text } if text == "ka"
        ));
        assert!(matches!(
            web.translate_composition_start(false, true),
            Some(InputEvent::CompositionStart)
        ));
        assert!(matches!(
            web.translate_composition_update("ka".to_owned(), false, true),
            Some(InputEvent::CompositionUpdate { text }) if text == "ka"
        ));
        assert!(matches!(
            native.translate_ime(&winit::event::Ime::Commit("क".to_owned()), true).as_slice(),
            [InputEvent::CompositionEnd { text }] if text == "क"
        ));
        assert!(matches!(
            web.translate_composition_end("क".to_owned(), false, true),
            Some(InputEvent::CompositionEnd { text }) if text == "क"
        ));

        assert!(matches!(
            web.translate_paste(Some("clip".to_owned()), true),
            Some(InputEvent::Paste { text }) if text == "clip"
        ));
        assert!(web
            .translate_paste(Some("clip".to_owned()), false)
            .is_none());
    }
}
