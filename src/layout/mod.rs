use core::fmt;
use slotmap::DefaultKey;
use std::collections::HashMap;
use taffy::{
    prelude::{Layout, Size},
    style::Style,
    style_helpers::TaffyMaxContent,
    Taffy,
};

enum Operation {
    Push(DefaultKey),
    Pop,
}

#[derive(Debug)]
struct GlobalLayout {
    layout: Layout,
    is_listening: bool,
}

#[derive(Default)]
pub struct LayoutTree {
    taffy: Taffy,
    global_layouts: HashMap<DefaultKey, GlobalLayout>,
}

impl LayoutTree {
    pub fn get(&self, key: DefaultKey) -> Option<&Layout> {
        self.global_layouts.get(&key).map(|global| &global.layout)
    }

    pub fn insert(&mut self, style: Style) -> DefaultKey {
        self.taffy.new_leaf(style).unwrap()
    }

    pub fn insert_with_children(&mut self, style: Style, children: &[DefaultKey]) -> DefaultKey {
        self.taffy.new_with_children(style, children).unwrap()
    }

    pub fn is_listening(&self, key: DefaultKey) -> bool {
        let global_layout = self.global_layouts.get(&key).unwrap();
        global_layout.is_listening
    }

    pub fn listen(&mut self, key: DefaultKey) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = true;
    }

    pub fn unlisten(&mut self, key: DefaultKey) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = false;
    }

    pub fn layout(&mut self, root: DefaultKey, mut listener: impl FnMut(DefaultKey, &Layout)) {
        taffy::compute_layout(&mut self.taffy, root, Size::MAX_CONTENT).unwrap();

        let mut stack = vec![Operation::Push(root)];
        let mut layouts: Vec<Layout> = vec![];
        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let mut layout = self.taffy.layout(key).unwrap().clone();
                    if let Some(parent_layout) = layouts.last() {
                        layout.location.x += parent_layout.location.x;
                        layout.location.y += parent_layout.location.y;
                    }

                    self.global_layouts
                        .entry(key)
                        .and_modify(|dst| {
                            if dst.is_listening
                                && (dst.layout.location != layout.location
                                    || dst.layout.order != layout.order
                                    || dst.layout.size != layout.size)
                            {
                                listener(key, &layout)
                            }

                            dst.layout = layout;
                        })
                        .or_insert_with(|| {
                            listener(key, &layout);
                            GlobalLayout {
                                layout,
                                is_listening: false,
                            }
                        });
                    layouts.push(layout);

                    stack.push(Operation::Pop);

                    let children = self.taffy.children(key).unwrap();
                    stack.extend(children.into_iter().map(|child| Operation::Push(child)));
                }
                Operation::Pop => {
                    layouts.pop();
                }
            }
        }
    }
}

impl fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayoutTree")
            .field("global_layouts", &self.global_layouts)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::LayoutTree;
    use taffy::{
        prelude::{Rect, Size},
        style::{LengthPercentage, Style},
    };

    #[test]
    fn it_works() {
        let mut tree = LayoutTree::default();

        let a = tree.insert(Style::default());

        let b = tree.insert_with_children(Style::default(), &[a]);

        let root = tree.insert_with_children(
            Style {
                size: Size::from_points(100., 100.),
                padding: Rect {
                    left: LengthPercentage::Points(100.),
                    right: LengthPercentage::Points(0.),
                    top: LengthPercentage::Points(0.),
                    bottom: LengthPercentage::Points(0.),
                },
                ..Default::default()
            },
            &[b],
        );
        tree.layout(root, |key, layout| {
            dbg!(key, layout);
        });
    }
}
