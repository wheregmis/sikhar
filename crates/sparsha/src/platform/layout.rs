use sparsha_layout::{ComputedLayout, LayoutTree, WidgetId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LayoutViewport {
    width: f32,
    height: f32,
}

impl LayoutViewport {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    fn definite_width(self) -> f32 {
        self.width.max(1.0)
    }

    fn definite_height(self) -> f32 {
        self.height.max(1.0)
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct LayoutSnapshot {
    entries: Vec<LayoutSnapshotEntry>,
}

impl LayoutSnapshot {
    fn capture(tree: &LayoutTree) -> Self {
        let mut entries = Vec::new();
        tree.traverse(|widget_id, layout, depth| {
            entries.push(LayoutSnapshotEntry {
                widget_id,
                layout: *layout,
                depth,
            });
        });
        Self { entries }
    }

    #[allow(dead_code)]
    pub(crate) fn entries(&self) -> &[LayoutSnapshotEntry] {
        &self.entries
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub(crate) struct LayoutSnapshotEntry {
    pub(crate) widget_id: WidgetId,
    pub(crate) layout: ComputedLayout,
    pub(crate) depth: usize,
}

pub(crate) fn compute_platform_layout(tree: &mut LayoutTree, viewport: LayoutViewport) {
    tree.compute_layout(viewport.definite_width(), viewport.definite_height());
}

#[allow(dead_code)]
pub(crate) fn compute_layout_snapshot(
    tree: &mut LayoutTree,
    viewport: LayoutViewport,
) -> LayoutSnapshot {
    compute_platform_layout(tree, viewport);
    LayoutSnapshot::capture(tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sparsha_layout::styles;

    #[derive(Clone, Debug, PartialEq)]
    struct StableLayoutEntry {
        depth: usize,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    fn representative_tree() -> LayoutTree {
        let mut tree = LayoutTree::new();
        let header = tree.new_leaf(styles::fixed(320.0, 48.0));
        let sidebar = tree.new_leaf(styles::fixed(96.0, 240.0));
        let content = tree.new_leaf(styles::fill());
        let body = tree.new_with_children(
            styles::with_gap(styles::flex_row(), 12.0),
            &[sidebar, content],
        );
        let root = tree.new_with_children(
            styles::with_gap(styles::with_padding(styles::flex_column(), 16.0), 8.0),
            &[header, body],
        );
        tree.set_root(root);
        tree
    }

    fn stable_entries(snapshot: &LayoutSnapshot) -> Vec<StableLayoutEntry> {
        snapshot
            .entries()
            .iter()
            .map(|entry| StableLayoutEntry {
                depth: entry.depth,
                x: entry.layout.bounds.x,
                y: entry.layout.bounds.y,
                width: entry.layout.bounds.width,
                height: entry.layout.bounds.height,
            })
            .collect()
    }

    fn native_runtime_layout_snapshot(width: f32, height: f32) -> LayoutSnapshot {
        let mut tree = representative_tree();
        compute_layout_snapshot(&mut tree, LayoutViewport::new(width, height))
    }

    fn web_runtime_layout_snapshot(width: f32, height: f32) -> LayoutSnapshot {
        let mut tree = representative_tree();
        compute_layout_snapshot(&mut tree, LayoutViewport::new(width, height))
    }

    #[test]
    fn adapter_clamps_empty_viewport_to_definite_layout_space() {
        let mut tree = LayoutTree::new();
        let root = tree.new_leaf(styles::fill());
        tree.set_root(root);
        let snapshot = compute_layout_snapshot(&mut tree, LayoutViewport::new(0.0, -20.0));
        let root = snapshot.entries().first().expect("root layout entry");
        assert_eq!(root.layout.bounds.width, 1.0);
        assert_eq!(root.layout.bounds.height, 1.0);
    }

    #[test]
    fn native_and_web_entrypoints_produce_identical_layout_snapshots() {
        let native = native_runtime_layout_snapshot(800.0, 600.0);
        let web = web_runtime_layout_snapshot(800.0, 600.0);
        assert_eq!(stable_entries(&native), stable_entries(&web));
    }
}
