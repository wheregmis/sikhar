# Element Composition API

Sparsha ships a GPUI-inspired element lane in `sparsha-elements` for declarative UI trees:

```rust
div()
    .flex()
    .flex_col()
    .gap_3()
    .p_4()
    .child(text("Hello").text_xl())
```

## Mapping

- Web routes built with `element_component()` render structural DOM through `DomTailwindRenderer` and Tailwind-style utility class names (mapped in `tailwind_map`; `hello-gpui` ships matching dev utilities in `index.html` instead of the Tailwind CDN).
- Native routes paint through the existing widget bridge (`element_to_widget`) and Taffy layout via `taffy_map`.
- `StyleRefinement` is the shared source of truth; `tailwind_map` and `taffy_map` translate it per target.

## Authoring

- Use `element_component().render(fn).call()` for stateful element pages.
- Use `element_root(|| tree)` for stateless element-only widgets.
- Leaf bridges: `TextWidgetElement`, `ButtonWidgetElement`, and `TextInputWidgetElement` wrap existing widgets when needed.

## Example

See `examples/hello-gpui` for hello-world and scrollable patterns on native and wasm.
