#![allow(dead_code)]

use std::str::FromStr;
use html_parser;

pub enum SelectorComponent {
	TagName(String)
}
impl SelectorComponent {
	fn matches(&self, element: &html_parser::tree_generation_2::HTMLElement) -> bool {
		match self {
			SelectorComponent::TagName(name) => element.is_tag(name)
		}
	}
}

pub struct Selector {
	components: Vec<SelectorComponent>
}
impl Selector {
	pub fn new(components: Vec<SelectorComponent>) -> Selector {
		Selector {
			components
		}
	}

	pub fn matches(&self, element: &html_parser::tree_generation_2::HTMLElement) -> bool {
		for component in &self.components {
			if !component.matches(element) {
				return false;
			}
		}

		return true;
	}
}
pub struct SelectorParseError {}
impl FromStr for Selector {
	type Err = SelectorParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Selector::new(vec![SelectorComponent::TagName(String::from(s))]))
	}
}