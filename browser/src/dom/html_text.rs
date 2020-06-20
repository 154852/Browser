#![allow(dead_code)]

use crate::dom::html_node;
use crate::styles;
use crate::html_render;
use std::rc::Rc;
use std::collections::HashMap;

pub struct TextNode {
	text: String
}

impl TextNode {
	pub fn new(text: String) -> TextNode {
		TextNode {text}
	}
}

impl html_node::HTMLNode for TextNode {
	fn compute_style(&self, _display: &glium::Display, _self_node: Rc<html_node::RenderBox>) -> Box<dyn html_render::RenderCall> {
		Box::new(html_render::BlankRenderCall::new())
	}

	fn prepare(&mut self, _global_applicable_styles: &HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>) {}
	
	fn accumulate_precomputed_styles(&self, _pcs: &mut HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>, _self_node: Rc<html_node::RenderBox>) {}
}