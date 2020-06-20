use tree;
use super::tag_parse::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct HTMLElement {
	attributes: HashMap<String, String>,
	tagname: String
}
impl HTMLElement {
	fn new(tagname: String, attributes: HashMap<String, String>) -> HTMLElement {
		HTMLElement {
			tagname, attributes
		}
	}

	pub fn attributes(&self) -> &HashMap<String, String> {
		&self.attributes
	}

	pub fn tagname(&self) -> &String {
		&self.tagname
	}

	pub fn is_tag(&self, name: &String) -> bool {
		self.tagname == *name
	}

	pub fn has_attribute(&self, name: &String) -> bool {
		self.attributes.contains_key(name)
	}

	pub fn get_attribute(&self, name: &String) -> Option<&String> {
		self.attributes.get(name)
	}

	pub fn get_attribute_unwrapped(&self, name: &String) -> &String {
		self.attributes.get(name).unwrap()
	}
}

#[derive(Debug)]
pub enum HTMLNode {
	Root,
	Text(String),
	Element(HTMLElement)
}

pub fn create_node_tree(tags: Vec<HTMLChild>) -> Rc<tree::TreeNode<HTMLNode>> {
	let root = tree::TreeNode::root(HTMLNode::Root);
	let mut parent = root.clone();

	for tag in tags {
		match tag {
			HTMLChild::StartTag(tag) => {
				let is_close = tag.is_self_close();
				let inner_element = HTMLNode::Element(HTMLElement::new(tag.name, tag.attrs));
				let element = tree::TreeNode::new(inner_element, parent.clone());
				parent.clone().add_child(element.clone());
				if !is_close { parent = element; }
			},
			HTMLChild::EndTag(_) => {
				if let Some(new_parent) = parent.get_parent() {
					parent = new_parent.clone();
				}
			},
			HTMLChild::Text(string) => {
				let inner_element = HTMLNode::Text(string);
				let element = tree::TreeNode::new(inner_element, parent.clone());
				parent.clone().add_child(element.clone());
			}
		}
	}

	root
}