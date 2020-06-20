use std::collections::HashMap;
use std::str::FromStr;

use html_parser;
use crate::html_render;
use crate::styles;
use crate::dom;
use crate::dom::html_node;
use crate::dom::selector;

pub struct WebApplication {
	renderer: html_render::Renderer,
	event_loop: glium::glutin::event_loop::EventLoop<()>
}
impl WebApplication {
	pub fn new() -> WebApplication {
		let (renderer, event_loop) = html_render::Renderer::create_context_and_loop("Browser");
		WebApplication {
			renderer, event_loop
		}
	}

	fn _start(event_loop: glium::glutin::event_loop::EventLoop<()>, mut renderer: html_render::Renderer) {
		event_loop.run(move |event, _, control_flow| {
			renderer.event(event, control_flow);
		});
	}

	pub fn start(self) {
		WebApplication::_start(self.event_loop, self.renderer);
	}

	pub fn load_html(&mut self, html: &str) {
		let dom_tree = html_parser::parse_html_new(html);
		let annotated_tree = dom_tree.clone().map(|node| {
			match &*node.value.borrow() {
				html_node::NodeT::Root => Box::new(dom::html_root::RootElement::new()) as Box<dyn html_node::HTMLNode>,
				html_node::NodeT::Element(_) => Box::new(dom::html_block_elements::BlockElement::new(node.clone())) as Box<dyn html_node::HTMLNode>,
				html_node::NodeT::Text(text) => Box::new(dom::html_text::TextNode::new(text.clone())) as Box<dyn html_node::HTMLNode>
			}
		});
	
		let styles_elements = dom_tree.clone().find(|node| {
			match &*node.value.borrow() {
				html_node::NodeT::Root => false,
				html_node::NodeT::Element(el) => el.is_tag(&String::from("style")),
				html_node::NodeT::Text(_) => false
			}
		});
		let total_style = styles_elements.iter().map(|element| {
			if let Some(element) = element.get_child(0) {
				if let html_node::NodeT::Text(text) = &*element.value.borrow() {
					text.clone()
				} else { panic!("Invalid CSS") }
			} else { String::new() }
		}).collect::<Vec<String>>().join("\n");
		let total_style_blocks = html_parser::parse_css(total_style.as_str());

		for rendercall in annotated_tree.map_linear(|node| {
			let mut relevant_styles = HashMap::new();
			for block in &total_style_blocks {
				let sel = selector::Selector::from_str(block.selector.as_str()).ok().unwrap();
				if node.value.borrow().matches(&sel) {
					styles::append_ref(&mut relevant_styles, block.as_ref());
				}
			}
			node.value.borrow_mut().prepare(&relevant_styles);
			node.value.borrow().compute_style(self.renderer.display(), node.clone())
		}) {
			self.renderer.add(rendercall);
		}

		self.renderer.queue();
	}
}
