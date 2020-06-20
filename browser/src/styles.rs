#![allow(dead_code)]

use html_parser;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum StyleName {
	Width,
	Height
}

#[derive(Debug)]
pub enum SizeType {
	Pixels(f32),
	Percent(f32)
}

#[derive(Debug)]
pub enum PreComputedStyleValue {
	Size(SizeType),
	Inherit,
	VDefault
}

#[derive(Debug)]
pub enum ComputedStyleValue {
	Size(f32), // px
	VDefault
}

fn parse_size(size: &String) -> PreComputedStyleValue {
	lazy_static! {
		static ref RE: regex::Regex = regex::Regex::new(r"([0-9.\-]+)(px|%)").unwrap();
	}

	if *size == String::from("inherit") {
		return PreComputedStyleValue::Inherit;
	} else if *size == String::from("default") {
		return PreComputedStyleValue::VDefault;
	}

	let m = RE.captures(size).expect("Invalid size value");
	match &m[2] {
		"px" => PreComputedStyleValue::Size(SizeType::Pixels(*&m[1].parse().unwrap())),
		"%" => PreComputedStyleValue::Size(SizeType::Percent(*&m[1].parse().unwrap())),
		_ => panic!("Invalid size unit {}", &m[2])
	}
}

fn compute_size(size: &PreComputedStyleValue) -> ComputedStyleValue {
	match size {
		PreComputedStyleValue::Size(size) => match size {
			SizeType::Pixels(v) => ComputedStyleValue::Size(*v),
			_ => ComputedStyleValue::Size(0.0)
		}
		PreComputedStyleValue::VDefault | _ => ComputedStyleValue::Size(0.0)
	}
}

pub fn append(styles: &mut HashMap<StyleName, Rc<PreComputedStyleValue>>, block: html_parser::css_parse::CSSBlock) {
	append_ref(styles, &block);
}

pub fn append_ref(styles: &mut HashMap<StyleName, Rc<PreComputedStyleValue>>, block: &html_parser::css_parse::CSSBlock) {
	for rule in &block.rules {
		match rule.name.as_str() {
			"width" => {styles.insert(StyleName::Width, Rc::new(parse_size(&rule.value)));},
			"height" => {styles.insert(StyleName::Height, Rc::new(parse_size(&rule.value)));},
			_ => { println!("Invalid style-rule: {}", rule.name) }
		}
	}
}

pub fn compute_styles(styles: &HashMap<StyleName, Rc<PreComputedStyleValue>>) -> HashMap<StyleName, ComputedStyleValue> {
	let mut new_map = HashMap::new();
	for (rule, value) in styles {
		match rule {
			StyleName::Width => new_map.insert(StyleName::Width, compute_size(value)),
			StyleName::Height => new_map.insert(StyleName::Height, compute_size(value))
		};
	}

	new_map
}