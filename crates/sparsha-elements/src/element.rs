//! Element tree types and composition helpers.

use crate::style::StyleRefinement;
use crate::styled::Styled;
use sparsha_core::Color;
use std::any::Any;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ELEMENT_ID: AtomicU64 = AtomicU64::new(1);

/// Stable identity for element nodes in a tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementId(u64);

impl ElementId {
    pub fn new() -> Self {
        Self(NEXT_ELEMENT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn from_u64(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

/// Phase-1 interaction handlers.
#[derive(Clone, Default)]
pub struct ElementHandlers {
    pub on_click: Option<Rc<dyn Fn()>>,
}

/// A node in the element tree.
#[derive(Clone)]
pub struct ElementNode {
    pub id: ElementId,
    pub style: StyleRefinement,
    pub handlers: ElementHandlers,
    pub kind: ElementKind,
}

/// Element variants supported in phase 1.
#[derive(Clone)]
pub enum ElementKind {
    Div { children: Vec<ElementNode> },
    Text { content: String },
    Button { label: String },
    /// Opaque widget bridge payload stored by higher layers.
    Any(Rc<dyn Any>),
}

impl ElementNode {
    pub fn div() -> Self {
        Self {
            id: ElementId::new(),
            style: StyleRefinement::default(),
            handlers: ElementHandlers::default(),
            kind: ElementKind::Div { children: Vec::new() },
        }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self {
            id: ElementId::new(),
            style: StyleRefinement::default(),
            handlers: ElementHandlers::default(),
            kind: ElementKind::Text {
                content: content.into(),
            },
        }
    }

    pub fn button(label: impl Into<String>) -> Self {
        Self {
            id: ElementId::new(),
            style: StyleRefinement::default(),
            handlers: ElementHandlers::default(),
            kind: ElementKind::Button {
                label: label.into(),
            },
        }
    }

    pub fn style(&self) -> &StyleRefinement {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }

    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.handlers.on_click = Some(Rc::new(handler));
        self
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<ElementNode>> {
        match &mut self.kind {
            ElementKind::Div { children } => Some(children),
            _ => None,
        }
    }

    pub fn is_scroll_container(&self) -> bool {
        matches!(
            (self.style.overflow_x, self.style.overflow_y),
            (Some(crate::style::OverflowAxis::Scroll), _)
                | (_, Some(crate::style::OverflowAxis::Scroll))
        )
    }
}

impl Styled for ElementNode {
    fn style_mut(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// GPUI-style div container.
pub struct Div {
    node: ElementNode,
}

impl Div {
    pub fn new() -> Self {
        Self {
            node: ElementNode::div(),
        }
    }

    pub fn into_node(self) -> ElementNode {
        self.node
    }

    pub fn style(&self) -> &StyleRefinement {
        self.node.style()
    }
}

impl Styled for Div {
    fn style_mut(&mut self) -> &mut StyleRefinement {
        self.node.style_mut()
    }
}

/// Types that can be converted into an element node.
pub trait IntoElement {
    fn into_element(self) -> ElementNode;
}

impl IntoElement for ElementNode {
    fn into_element(self) -> ElementNode {
        self
    }
}

impl IntoElement for Div {
    fn into_element(self) -> ElementNode {
        self.node
    }
}

impl IntoElement for &str {
    fn into_element(self) -> ElementNode {
        ElementNode::text(self)
    }
}

impl IntoElement for String {
    fn into_element(self) -> ElementNode {
        ElementNode::text(self)
    }
}

impl IntoElement for Color {
    fn into_element(self) -> ElementNode {
        ElementNode::div().bg(self).into_element()
    }
}

/// Parent containers can accept one or many children.
pub trait ParentElement {
    fn child(mut self, child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.push_child(child.into_element());
        self
    }

    fn children<I>(mut self, children: I) -> Self
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: IntoElement,
    {
        for child in children {
            self.push_child(child.into_element());
        }
        self
    }

    fn push_child(&mut self, child: ElementNode);
}

impl ParentElement for Div {
    fn push_child(&mut self, child: ElementNode) {
        if let Some(children) = self.node.children_mut() {
            children.push(child);
        }
    }
}

impl ParentElement for ElementNode {
    fn push_child(&mut self, child: ElementNode) {
        if let Some(children) = self.children_mut() {
            children.push(child);
        }
    }
}

/// Authoring entrypoint for div containers.
pub fn div() -> Div {
    Div::new()
}

/// Fluent text element.
pub struct TextElement {
    node: ElementNode,
}

impl TextElement {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            node: ElementNode::text(content),
        }
    }

    pub fn into_node(self) -> ElementNode {
        self.node
    }
}

impl Styled for TextElement {
    fn style_mut(&mut self) -> &mut StyleRefinement {
        self.node.style_mut()
    }
}

impl IntoElement for TextElement {
    fn into_element(self) -> ElementNode {
        self.node
    }
}

pub fn text(content: impl Into<String>) -> TextElement {
    TextElement::new(content)
}

/// Fluent button element.
pub struct ButtonElement {
    node: ElementNode,
}

impl ButtonElement {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            node: ElementNode::button(label),
        }
    }

    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.node = self.node.on_click(handler);
        self
    }

    pub fn into_node(self) -> ElementNode {
        self.node
    }
}

impl Styled for ButtonElement {
    fn style_mut(&mut self) -> &mut StyleRefinement {
        self.node.style_mut()
    }
}

impl IntoElement for ButtonElement {
    fn into_element(self) -> ElementNode {
        self.node
    }
}

pub fn button(label: impl Into<String>) -> ButtonElement {
    ButtonElement::new(label)
}
