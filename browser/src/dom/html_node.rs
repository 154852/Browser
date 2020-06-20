#![allow(dead_code)]

use std::collections::HashMap;
use crate::html_render;
use std::fmt;
use crate::styles;
use std::rc::Rc;
use tree;
use super::selector;

pub type Node = tree::TreeNode<html_parser::tree_generation_2::HTMLNode>;
pub type NodeT = html_parser::tree_generation_2::HTMLNode;
pub type RenderBox = tree::TreeNode<Box<dyn HTMLNode>>;

pub trait HTMLNode {
	fn compute_style(&self, display: &glium::Display, self_node: Rc<RenderBox>) -> Box<dyn html_render::RenderCall>;
	fn name(&self) -> String { String::from("HTMLNode") }
	fn prepare(&mut self, global_applicable_styles: &HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>);
	fn accumulate_precomputed_styles(&self, pcs: &mut HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>, self_node: Rc<RenderBox>);
	#[allow(unused_variables)]
	fn matches(&self, selector: &selector::Selector) -> bool { false }
}

impl fmt::Debug for dyn HTMLNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct(self.name().as_str()).finish()
	}
}