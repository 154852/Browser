#![allow(dead_code)]

#[macro_use] extern crate glium;
#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::collections::HashMap;

use html_parser;
mod html_render;
mod dom;
use dom::html_node;
use dom::selector;
mod styles;
mod web_frame;

fn main() {
    // let dom_tree = html_parser::parse_html_new("<style>a { width: inherit; height: default; } </style> <style>div { width: 100px; } \na {height: 100px;}</style><div><a></a></div>");
    // let annotated_tree = dom_tree.clone().map(|node| {
    //     match &*node.value.borrow() {
    //         html_node::NodeT::Root => Box::new(dom::html_root::RootElement::new()) as Box<dyn html_node::HTMLNode>,
    //         html_node::NodeT::Element(_) => Box::new(dom::html_block_elements::BlockElement::new(node.clone())) as Box<dyn html_node::HTMLNode>,
    //         html_node::NodeT::Text(text) => Box::new(dom::html_text::TextNode::new(text.clone())) as Box<dyn html_node::HTMLNode>
    //     }
    // });

    // let styles_elements = dom_tree.clone().find(|node| {
    //     match &*node.value.borrow() {
    //         html_node::NodeT::Root => false,
    //         html_node::NodeT::Element(el) => el.is_tag(&String::from("style")),
    //         html_node::NodeT::Text(_) => false
    //     }
    // });
    // let total_style = styles_elements.iter().map(|element| {
    //     if let Some(element) = element.get_child(0) {
    //         if let html_node::NodeT::Text(text) = &*element.value.borrow() {
    //             text.clone()
    //         } else { panic!("Invalid CSS") }
    //     } else { String::new() }
    // }).collect::<Vec<String>>().join("\n");
    // let total_style_blocks = html_parser::parse_css(total_style.as_str());

    // let (mut renderer, event_loop) = html_render::Renderer::create_context_and_loop("Browser");

    // for rendercall in annotated_tree.map_linear(|node| {
    //     let mut relevant_styles = HashMap::new();
    //     for block in &total_style_blocks {
    //         let sel = selector::Selector::from_str(block.selector.as_str()).ok().unwrap();
    //         if node.value.borrow().matches(&sel) {
    //             styles::append_ref(&mut relevant_styles, block.as_ref());
    //         }
    //     }
    //     node.value.borrow_mut().prepare(&relevant_styles);
    //     node.value.borrow().compute_style(renderer.display(), node.clone())
    // }) {
    //     renderer.add(rendercall);
    // }

    // renderer.queue();
    // event_loop.run(move |event, _, control_flow| {
    //     renderer.event(event, control_flow);
    // });
    let mut app = web_frame::WebApplication::new();
    app.load_html("<div style=\"width: 100px; height: 20px;\"></div>");
    app.start();
}
