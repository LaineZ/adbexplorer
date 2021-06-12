/// Slightly modified version of 'shoji' layout engine
/// Originally created by Richard Anaya under MIT license 

extern crate alloc;
use alloc::vec::Vec;

use generational_arena::{Arena, Index};

pub type NodeIndex = Index;

pub struct LayoutStyle {
    pub direction: Direction,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle {
            direction: Direction::LeftRight,
        }
    }
}

pub enum Direction {
    TopBottom,
    LeftRight,
}

#[derive(Debug, PartialEq)]
pub struct Layout {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct Node {
    pub layout: Option<Layout>,
    pub style: LayoutStyle,
    pub children: Vec<NodeIndex>,
}

pub struct LayoutEngine {
    nodes: Arena<Node>,
}

pub struct LayoutSize {
    width: Option<i32>,
    height: Option<i32>,
}

impl LayoutSize {
    pub fn new(w: i32, h: i32) -> LayoutSize {
        LayoutSize {
            width: Some(w),
            height: Some(h),
        }
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        LayoutEngine {
            nodes: Arena::new(),
        }
    }

    pub fn new_node(&mut self, style: LayoutStyle, children: Vec<NodeIndex>) -> NodeIndex {
        self.nodes.insert(Node {
            layout: None,
            style,
            children,
        })
    }

    pub fn get_node(&mut self, node_index: NodeIndex) -> &mut Node {
        &mut self.nodes[node_index]
    }

    pub fn get_layout(&self, i: NodeIndex) -> Result<&Layout, &'static str> {
        match &self.nodes[i].layout {
            Some(l) => Ok(l),
            None => Err("layout has not been calculated yet"),
        }
    }

    pub fn compute_layout(
        &mut self,
        node_index: NodeIndex,
        s: LayoutSize,
    ) -> Result<(), &'static str> {
        self.compute_layout_helper(0, 0, node_index, s)
    }

    fn compute_layout_helper(
        &mut self,
        x: i32,
        y: i32,
        node_index: NodeIndex,
        s: LayoutSize,
    ) -> Result<(), &'static str> {
        let node = self.get_node(node_index);
        node.layout = Some(Layout {
            x,
            y,
            w: s.width.ok_or("cannot create width from undefined value")?,
            h: s.height
                .ok_or("cannot create height from undefined value")?,
        });
        let children = node.children.clone();
        let num_children = children.len();
        if num_children == 0 {
            // do nothing
            Ok(())
        } else if num_children == 1 {
            self.compute_layout_helper(x, y, children[0], s)?;
            Ok(())
        } else {
            match node.style.direction {
                Direction::LeftRight => {
                    let width = s.width;
                    let height = s.height;
                    match width {
                        Some(w) => {
                            let child_width = w / children.len() as i32;
                            for (i, c) in children.iter().enumerate() {
                                self.compute_layout_helper(
                                    x + i as i32 * child_width,
                                    y,
                                    *c,
                                    LayoutSize {
                                        width: Some(child_width),
                                        height,
                                    },
                                )?;
                            }
                            Ok(())
                        }
                        None => Err("cannot compute layout of LeftRight without defined width"),
                    }
                }
                Direction::TopBottom => {
                    let width = s.width;
                    let height = s.height;
                    match height {
                        Some(h) => {
                            let child_height = h / children.len() as i32;
                            for (i, c) in children.iter().enumerate() {
                                self.compute_layout_helper(
                                    x,
                                    y + i as i32 * child_height,
                                    *c,
                                    LayoutSize {
                                        width,
                                        height: Some(child_height),
                                    },
                                )?;
                            }
                            Ok(())
                        }
                        None => Err("cannot compute layout of TopBottom without defined width"),
                    }
                }
            }
        }
    }
}
