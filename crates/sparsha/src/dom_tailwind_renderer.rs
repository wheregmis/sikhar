//! Structural DOM renderer that maps element trees to Tailwind classes.

#![cfg(target_arch = "wasm32")]

use sparsha_elements::{
    tailwind_map::class_string_from_refinement, ElementId, ElementKind, ElementNode,
};
use sparsha_widgets::ElementDomSnapshot;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlButtonElement, HtmlElement, EventTarget};

/// Retained Tailwind DOM layer for element-first routes.
pub struct DomTailwindRenderer {
    root: HtmlElement,
    pool: Vec<HtmlElement>,
    active_nodes: usize,
}

impl DomTailwindRenderer {
    pub fn mount_under(
        parent: &HtmlElement,
        document: &Document,
    ) -> Result<Self, wasm_bindgen::JsValue> {
        let root = document.create_element("div")?.dyn_into::<HtmlElement>()?;
        root.set_class_name("sparsha-tailwind-root flex flex-col w-full h-full min-h-0");
        set_style(&root, "position", "relative")?;
        set_style(&root, "width", "100%")?;
        set_style(&root, "height", "100%")?;
        set_style(&root, "overflow", "hidden")?;
        parent.append_child(&root)?;
        Ok(Self {
            root,
            pool: Vec::new(),
            active_nodes: 0,
        })
    }

    pub fn root(&self) -> &HtmlElement {
        &self.root
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), wasm_bindgen::JsValue> {
        set_style(
            &self.root,
            "display",
            if visible { "flex" } else { "none" },
        )
    }

    pub fn render_snapshot(
        &mut self,
        snapshot: &ElementDomSnapshot,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Result<(), wasm_bindgen::JsValue> {
        set_style(&self.root, "width", &format!("{}px", viewport_width.max(1.0)))?;
        set_style(&self.root, "height", &format!("{}px", viewport_height.max(1.0)))?;
        while self.root.first_child().is_some() {
            if let Some(child) = self.root.first_child() {
                let _ = self.root.remove_child(&child);
            }
        }
        self.active_nodes = 0;
        self.render_node(&snapshot.root, &self.root.clone())?;
        for index in self.active_nodes..self.pool.len() {
            set_style(&self.pool[index], "display", "none")?;
        }
        Ok(())
    }

    fn render_node(
        &mut self,
        node: &ElementNode,
        parent: &HtmlElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let index = self.active_nodes;
        self.active_nodes += 1;

        let (tag, text) = match &node.kind {
            ElementKind::Div { .. } => ("div", None),
            ElementKind::Text { content } => ("span", Some(content.as_str())),
            ElementKind::Button { label } => ("button", Some(label.as_str())),
            ElementKind::Any(_) => ("div", None),
        };

        let element = self.ensure_node(index, tag)?;
        element.set_class_name(&class_string_from_refinement(&node.style));
        element.set_attribute(
            "data-sparsha-element-id",
            &node.id.as_u64().to_string(),
        )?;
        set_style(
            &element,
            "display",
            if matches!(node.kind, ElementKind::Div { .. }) {
                "flex"
            } else if matches!(node.kind, ElementKind::Text { .. }) {
                "block"
            } else {
                "inline-block"
            },
        )?;
        if node.handlers.on_click.is_some() {
            element.set_attribute("data-sparsha-click", "true")?;
        } else {
            let _ = element.remove_attribute("data-sparsha-click");
        }

        match (&node.kind, text) {
            (ElementKind::Button { .. }, Some(label)) => {
                let button = element.dyn_ref::<HtmlButtonElement>().expect("button element");
                button.set_type("button");
                button.set_text_content(Some(label));
            }
            (_, Some(label)) => element.set_text_content(Some(label)),
            _ => element.set_text_content(None),
        }

        parent.append_child(&element)?;

        if let ElementKind::Div { children } = &node.kind {
            for child in children {
                self.render_node(child, &element)?;
            }
        }

        Ok(())
    }

    fn ensure_node(&mut self, index: usize, tag: &str) -> Result<HtmlElement, wasm_bindgen::JsValue> {
        if index < self.pool.len() {
            let existing = &self.pool[index];
            if existing.tag_name().eq_ignore_ascii_case(tag) {
                return Ok(existing.clone());
            }
            let document = existing
                .owner_document()
                .ok_or_else(|| wasm_bindgen::JsValue::from_str("missing owner document"))?;
            let replacement = document.create_element(tag)?.dyn_into::<HtmlElement>()?;
            self.pool[index] = replacement;
            return Ok(self.pool[index].clone());
        }
        let document = self
            .root
            .owner_document()
            .ok_or_else(|| wasm_bindgen::JsValue::from_str("missing owner document"))?;
        let node = document.create_element(tag)?.dyn_into::<HtmlElement>()?;
        self.pool.push(node);
        Ok(self.pool[index].clone())
    }
}

/// Resolve the element id from a DOM event target (click / pointer-up).
pub fn element_id_from_target(target: Option<EventTarget>) -> Option<ElementId> {
    let target = target?;
    let element: web_sys::Element = target.dyn_into().ok()?;
    let node = element
        .closest("[data-sparsha-element-id]")
        .ok()
        .flatten()?;
    let id = node.get_attribute("data-sparsha-element-id")?.parse().ok()?;
    Some(ElementId::from_u64(id))
}

/// Invoke the `on_click` handler for the element with `id`, if present.
pub fn dispatch_element_click(snapshot: &ElementDomSnapshot, id: ElementId) -> bool {
    if let Some(handler) = find_click_handler(&snapshot.root, id) {
        handler();
        return true;
    }
    false
}

fn find_click_handler(
    node: &ElementNode,
    id: ElementId,
) -> Option<std::rc::Rc<dyn Fn()>> {
    if node.id == id {
        return node.handlers.on_click.clone();
    }
    if let ElementKind::Div { children } = &node.kind {
        for child in children {
            if let Some(handler) = find_click_handler(child, id) {
                return Some(handler);
            }
        }
    }
    None
}

fn set_style(element: &HtmlElement, property: &str, value: &str) -> Result<(), wasm_bindgen::JsValue> {
    element.style().set_property(property, value).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use sparsha_elements::{div, button, ParentElement, Styled};
    use sparsha_elements::tailwind_map::class_string_from_refinement;
    use sparsha_core::Color;

    #[test]
    fn tailwind_class_string_for_nested_div() {
        let tree = div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(Color::from_hex(0x111827))
            .child(div().text_xl().child("Hello GPUI-style Sparsha"))
            .into_element();
        let classes = class_string_from_refinement(tree.style());
        assert!(classes.contains("flex"));
        assert!(classes.contains("gap-3"));
    }

    #[test]
    fn button_node_maps_button_tag() {
        let node = button("Increment").into_element();
        assert!(matches!(node.kind, ElementKind::Button { .. }));
    }
}
