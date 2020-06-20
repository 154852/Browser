#![allow(dead_code)]

use crate::dom::html_node;
use crate::styles;
use crate::html_render;
use html_parser;
use std::rc::Rc;
use std::collections::HashMap;
use crate::dom::selector;

pub struct BlockElement {
	element: Rc<html_node::Node>,
	pre_computed_styles: HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>
}

impl<'a> BlockElement {
	pub fn new(element: Rc<html_node::Node>) -> BlockElement {
		BlockElement {
			element,
			pre_computed_styles: HashMap::new(),
		}
	}
}

impl html_node::HTMLNode for BlockElement {
	fn compute_style(&self, display: &glium::Display, self_node: Rc<html_node::RenderBox>) -> Box<dyn html_render::RenderCall> {
		let mut pre_computed = HashMap::new();
		self.accumulate_precomputed_styles(&mut pre_computed, self_node);

		let rect_render = html_render::RectRender::new(display);
		let computed = styles::compute_styles(&pre_computed);

		let mut width: f32 = 0.0; let mut height: f32 = 0.0;
		if let Some(styles::ComputedStyleValue::Size(w)) = computed.get(&styles::StyleName::Width) { width = *w; }
		if let Some(styles::ComputedStyleValue::Size(h)) = computed.get(&styles::StyleName::Height) { height = *h; }

		let mut detail = html_render::RectDetail::new(0.0, 0.0, width, height);
		detail.opaque(0.0, 0.0, 0.0);

		Box::new(html_render::RectRenderCall::new(Rc::new(detail), Rc::new(rect_render)))
	}

	fn matches(&self, selector: &selector::Selector) -> bool {
		if let html_node::NodeT::Element(el) = &*self.element.value.borrow() {
			selector.matches(el)
		} else { panic!("Unreachable") }
	}

	fn prepare(&mut self, global_applicable_styles: &HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>) {
		if let html_node::NodeT::Element(el) = &*self.element.value.borrow() {
			if let Some(style_attr) = el.get_attribute(&String::from("style")) {
				styles::append(&mut self.pre_computed_styles, html_parser::parse_css_inner(style_attr.as_str()));
			}
		}

		for (key, value) in global_applicable_styles {
			self.pre_computed_styles.entry(key.clone()).or_insert(value.clone());
		}
	}
	
	fn accumulate_precomputed_styles(&self, pcs: &mut HashMap<styles::StyleName, Rc<styles::PreComputedStyleValue>>, self_node: Rc<html_node::RenderBox>) {
		for (key, value) in &self.pre_computed_styles {
			if !pcs.contains_key(key) || (std::mem::discriminant(pcs.get(key).unwrap().as_ref()) == std::mem::discriminant(&styles::PreComputedStyleValue::Inherit)) {
				pcs.insert(key.clone(), value.clone());
			}
		}

		let parent = self_node.get_parent().unwrap();
		let parent_ref = &*parent.value.borrow();
		parent_ref.accumulate_precomputed_styles(pcs, self_node.get_parent().unwrap());
	}
}