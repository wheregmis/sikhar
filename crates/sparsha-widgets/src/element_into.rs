//! IntoElement adapters for existing leaf widgets.

use crate::{Button, Text, TextInput};
use sparsha_elements::{ElementKind, ElementNode, IntoElement};
use std::rc::Rc;

fn widget_leaf(widget: Box<dyn crate::Widget>) -> ElementNode {
    let mut node = ElementNode::div();
    node.kind = ElementKind::Any(Rc::new(widget));
    node
}

/// Wrap a built [`Text`] widget as an element leaf.
pub struct TextWidgetElement(pub Text);

impl IntoElement for TextWidgetElement {
    fn into_element(self) -> ElementNode {
        widget_leaf(Box::new(self.0))
    }
}

/// Wrap a built [`Button`] widget as an element leaf.
pub struct ButtonWidgetElement(pub Button);

impl IntoElement for ButtonWidgetElement {
    fn into_element(self) -> ElementNode {
        widget_leaf(Box::new(self.0))
    }
}

/// Wrap a built [`TextInput`] widget as an element leaf.
pub struct TextInputWidgetElement(pub TextInput);

impl IntoElement for TextInputWidgetElement {
    fn into_element(self) -> ElementNode {
        widget_leaf(Box::new(self.0))
    }
}
