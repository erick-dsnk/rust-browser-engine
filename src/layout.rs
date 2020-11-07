use std::fmt;

use crate::css::{
    Unit,
    Value,
};
use crate::style::{
    Display,
    StyledNode,
};

#[derive(Clone)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    box_type: BoxType,
    pub styled_node: &'a StyledNode<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub content: Rectangle,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    margin: EdgeSizes,
    current: Rectangle,
}

#[derive(Clone, Copy, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Default)]
pub struct EdgeSizes {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

#[derive(Clone, Copy)]
pub enum BoxType {
    Block,
    InlineBlock,
    Inline,
    Anonymous,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType, styled_node: &'a StyledNode) -> LayoutBox<'a> {
        return LayoutBox {
            dimensions: Default::default(),
            box_type,
            styled_node,
            children: Vec::new(),
        }
    }

    fn layout(&mut self, b_box: Dimensions) {
        match self.box_type {
            BoxType::Block => self.layout_block(b_box),
            BoxType::Inline => self.layout_block(b_box),
            BoxType::InlineBlock => self.layout_inline_block(b_box),
            BoxType::Anonymous => {},
        }
    }

    fn layout_block(&mut self, b_box: Dimensions) {
        self.calculate_width(b_box);
        self.calculate_pos(b_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_width(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        let width = get_abs_num(s, b_box, "width").unwrap_or(0.0);
        let margin_left = s.value("margin-left");
        let margin_right = s.value("margin-right");

        let margin_left_num = match margin_left {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        let margin_right_num = match margin_right {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        d.border.left = s.num_or("border-left-width", 0.0);
        d.border.right = s.num_or("border-left-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);

        let total = width + margin_left_num + margin_right_num + d.border.left + d.border.right + d.padding.left + d.padding.right;

        let underflow = b_box.content.width - total;

        match (width, margin_left, margin_right) {
            (0.0, _, _) => {
                if underflow >= 0.0 {
                    d.content.width = underflow;
                    d.margin.right = margin_right_num;
                } else {
                    d.margin.right = margin_right_num + underflow;
                    d.content.width = width;
                }

                d.margin.left = margin_left_num;
            },
            (w, None, Some(_)) if w != 0.0 => {
                d.margin.left = underflow;
                d.margin.right = margin_right_num;
                d.content.width = w;
            },
            (w, Some(_), None) if w != 0.0 => {
                d.margin.right = underflow;
                d.margin.left = margin_left_num;
                d.content.width = w;
            },
            (w, None, None) if w != 0.0 {
                d.margin.left = underflow / 2.0;
                d.margin.right = underflow / 2.0;
                d.content.width = w;
            },
            (_, _, _) => {
                d.margin.right = margin_right_num + underflow;
                d.margin.left = margin_left_num;
                d.content.width = width;
            },
        }
    }

    fn calculate_pos(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.margin.top = s.num_or("margin-top", 0.0);
        d.margin.bottom = s.num_or("margin-bottom", 0.0);
        d.border.top = s.num_or("border-top-width", 0.0);
        d.border.bottom = s.num_or("border-bottom-width", 0.0);
        d.padding.top = s.num_or("padding-top", 0.0);
        d.padding.bottom = s.num_or("padding-bottom", 0.0);

        d.content.x = b_box.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top + d.padding.top;
    }

    fn calculate_height(&mut self) {
        self.styled_node.value("height").map_or((), |h| match **h {
            Value::Length(n, _) => self.dimensions.content.height = n,
            _ => {},
        });
    }

    fn layout_children(&mut self) {
        let d = &mut self.dimensions;
        let mut max_child_height = 0.0;

        let mut prevBoxType = BoxType::Block;

        for child in &mut self.children {
            match prevBoxType {
                BoxType::InlineBlock => match child.box_type {
                    BoxType::Block => {
                        d.content.height += max_child_height;
                        d.current.x = 0.0;
                    },
                    _ => {},
                },
                _ => {},
            }

            child.layout(*d);

            let new_height = child.dimensions.margin_box().height;
        }
    }

    fn layout_inline_block(&mut self, b_box: Dimensions) {
        self.calculate_inline_width(b_box);
        self.calculate_inline_pos(b_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_inline_width(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.content.width = get_abs_num(s, b_box, "width").unwrap_or(0.0);
        d.margin.left = s.num_or("margin-left", 0.0);
        d.margin.right = s.num_or("margin-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left-width", 0.0);
        d.border.right = s.num_or("border-right-width", 0.0);
    }

    fn calculate_inline_pos(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.margin.top = s.num_or("margin-top", 0.0);
        d.margin.bottom = s.num_or("margin-bottom", 0.0);
        d.padding.top = s.num_or("padding-top", 0.0);
        d.padding.bottom = s.num_or("padding-bottom", 0.0);
        d.border.top = s.num_or("border-top-width", 0.0);
        d.border.bottom = s.num_or("border-bottom-width", 0.0);

        d.content.x = b_box.content.x + b_box.current.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.y + b_box.current.y + d.margin.top + d.border.top + d.padding.top;
    }

}