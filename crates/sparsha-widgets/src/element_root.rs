//! Bridge element trees into Sparsha widgets.

use crate::{
    current_theme, Button, ButtonVariant, Container, CrossAxisAlignment, IntoWidget,
    MainAxisAlignment, Scroll, Text, TextAlign, TextOverflow, Widget,
};
use sparsha_core::Color;
use sparsha_elements::{
    taffy_map::style_from_refinement, ElementKind, ElementNode, ElementTextAlign, FontWeight,
    IntoElement,
};
use sparsha_input::InputEvent;
use sparsha_layout::WidgetId;
use sparsha_layout::taffy::prelude::Style;
use std::rc::Rc;

/// Snapshot used by the web Tailwind DOM renderer.
#[derive(Clone)]
pub struct ElementDomSnapshot {
    pub root: ElementNode,
}

/// Widget host that rebuilds from an element tree and paints through the widget bridge.
pub struct ElementRoot<F> {
    id: WidgetId,
    render: F,
    element: ElementNode,
    children: Vec<Box<dyn Widget>>,
}

impl<F> ElementRoot<F> {
    pub fn new(mut render: F) -> Self
    where
        F: FnMut() -> ElementNode,
    {
        let element = render();
        let child = element_to_widget(&element);
        Self {
            id: WidgetId::default(),
            render,
            element,
            children: vec![child],
        }
    }

    pub fn element_snapshot(&self) -> ElementDomSnapshot {
        ElementDomSnapshot {
            root: self.element.clone(),
        }
    }

}

impl<F> ElementRoot<F> {
    fn element_dom_enabled(&self) -> bool {
        true
    }
}

/// Build an element-root widget from a render closure.
pub fn element_root<F>(render: F) -> ElementRoot<F>
where
    F: FnMut() -> ElementNode + 'static,
{
    ElementRoot::new(render)
}

impl<F> Widget for ElementRoot<F>
where
    F: FnMut() -> ElementNode + 'static,
{
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> Style {
        style_from_refinement(&self.element.style)
    }

    fn rebuild(&mut self, ctx: &mut crate::BuildContext) {
        self.element = (self.render)();
        self.children = vec![element_to_widget(&self.element)];
        if let Some(child) = self.children.first_mut() {
            child.rebuild(ctx);
        }
    }

    fn paint(&self, ctx: &mut crate::PaintContext) {
        if let Some(child) = self.children.first() {
            child.paint(ctx);
        }
    }

    fn paint_after_children(&self, ctx: &mut crate::PaintContext) {
        if let Some(child) = self.children.first() {
            child.paint_after_children(ctx);
        }
    }

    fn event(&mut self, ctx: &mut crate::EventContext, event: &InputEvent) {
        if let Some(child) = self.children.first_mut() {
            child.event(ctx, event);
        }
        dispatch_element_handlers(&self.element, event);
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
    }

    fn element_dom_snapshot(&self) -> Option<crate::ElementDomSnapshot> {
        Some(self.element_snapshot())
    }

    fn uses_element_dom(&self) -> bool {
        self.element_dom_enabled()
    }
}

fn dispatch_element_handlers(node: &ElementNode, event: &InputEvent) {
    if let InputEvent::PointerUp { .. } = event {
        if let Some(handler) = &node.handlers.on_click {
            handler();
        }
    }
    if let ElementKind::Div { children } = &node.kind {
        for child in children {
            dispatch_element_handlers(child, event);
        }
    }
}

pub fn element_to_widget(node: &ElementNode) -> Box<dyn Widget> {
    if node.is_scroll_container() {
        let inner = element_to_widget_inner(node);
        return Scroll::vertical(inner).into_widget();
    }
    element_to_widget_inner(node)
}

fn element_to_widget_inner(node: &ElementNode) -> Box<dyn Widget> {
    match &node.kind {
        ElementKind::Div { children } => div_to_container(node, children),
        ElementKind::Text { content } => text_to_widget(node, content),
        ElementKind::Button { label } => button_to_widget(node, label),
        ElementKind::Any(_) => Container::column().into_widget(),
    }
}

fn div_to_container(node: &ElementNode, children: &[ElementNode]) -> Box<dyn Widget> {
    let refinement = &node.style;
    let direction = refinement.flex_direction.unwrap_or_default();
    let mut container = match direction {
        sparsha_elements::FlexDirection::Row => Container::row(),
        sparsha_elements::FlexDirection::Column => Container::column(),
    };

    if refinement.display_flex {
        container = container.direction(match direction {
            sparsha_elements::FlexDirection::Row => sparsha_layout::taffy::FlexDirection::Row,
            sparsha_elements::FlexDirection::Column => {
                sparsha_layout::taffy::FlexDirection::Column
            }
        });
    }

    if let Some(gap) = refinement.gap {
        container = container.gap(gap.value());
    }
    if let Some(padding) = refinement.padding {
        container = container.padding(padding.value());
    }
    if refinement.size_full {
        container = container.fill();
    } else {
        if refinement.fill_width {
            container = container.fill_width();
        }
        if refinement.fill_height {
            container = container.fill_height();
        }
    }
    if let Some(background) = refinement.background {
        container = container
            .background(background)
            .corner_radius(refinement.border_radius.map(|v| v.value()).unwrap_or(0.0))
            .border(
                refinement.border_width.map(|v| v.value()).unwrap_or(0.0),
                refinement.border_color.unwrap_or(Color::TRANSPARENT),
            );
    }

    container = container
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Stretch);

    for child in children {
        container = container.child(element_to_widget(child));
    }

    container.into_widget()
}

fn text_to_widget(node: &ElementNode, content: &str) -> Box<dyn Widget> {
    let refinement = &node.style;
    let theme = current_theme();
    Text::builder()
        .content(content)
        .color(refinement.text_color.unwrap_or(theme.text_color()))
        .font_size(
            refinement
                .font_size
                .map(|size| size.value())
                .unwrap_or(theme.body_size()),
        )
        .bold(refinement.font_weight == Some(FontWeight::Bold))
        .align(match refinement.text_align {
            Some(ElementTextAlign::Center) => TextAlign::Center,
            Some(ElementTextAlign::Right) => TextAlign::Right,
            _ => TextAlign::Left,
        })
        .fill_width(true)
        .overflow(TextOverflow::Clip)
        .build()
        .into_widget()
}

fn button_to_widget(node: &ElementNode, label: &str) -> Box<dyn Widget> {
    let refinement = &node.style;
    match node.handlers.on_click.clone() {
        Some(handler) => Button::builder()
            .label(label)
            .variant(ButtonVariant::Primary)
            .background(refinement.background.unwrap_or(current_theme().primary_color()))
            .on_click(move || handler())
            .build()
            .into_widget(),
        None => Button::builder()
            .label(label)
            .variant(ButtonVariant::Primary)
            .background(refinement.background.unwrap_or(current_theme().primary_color()))
            .build()
            .into_widget(),
    }
}

/// Wrap an existing widget as an element leaf for mixed trees.
pub struct WidgetElement {
    node: ElementNode,
    widget: Box<dyn Widget>,
}

impl WidgetElement {
    pub fn new(widget: impl IntoWidget) -> Self {
        Self {
            node: ElementNode::div(),
            widget: widget.into_widget(),
        }
    }
}

impl IntoElement for WidgetElement {
    fn into_element(self) -> ElementNode {
        let mut node = self.node;
        node.kind = ElementKind::Any(Rc::new(self.widget));
        node
    }
}
