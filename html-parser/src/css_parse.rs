use std::rc::Rc;
use regex::Regex;

#[derive(Debug)]
pub struct CSSRule {
	pub name: String,
	pub value: String
}

impl CSSRule {
	fn new(name: String, value: String) -> CSSRule {
		CSSRule {
			name, value
		}
	}
}

#[derive(Debug)]
pub struct CSSBlock {
	pub rules: Vec<Rc<CSSRule>>,
	pub selector: String
}

impl CSSBlock {
	fn new() -> CSSBlock {
		CSSBlock {
			rules: vec![],
			selector: String::new()
		}
	}
}

pub fn create_block(css: &str) -> CSSBlock {
	let rule_re = Regex::new(r"([a-z\-]+)\s*:\s*([^;]+);").unwrap();
	let mut block = CSSBlock::new();

	block.rules = rule_re.captures_iter(css).map(|x| {
		Rc::new(CSSRule::new(String::from(&x[1]), String::from((&x[2]).trim())))
	}).collect();

	block
}

pub fn create_blocks(css: &str) -> Vec<Rc<CSSBlock>> {
	let block_re = Regex::new(r"([a-zA-Z_0-9.\->\s_#]+)\s*\{([^}]+)\}").unwrap();
	let rule_re = Regex::new(r"([a-z\-]+)\s*:\s*([^;]+);").unwrap();

	block_re.captures_iter(css).map(|x| {
		let mut block = CSSBlock::new();
		block.selector = String::from((&x[1]).trim());

		block.rules = rule_re.captures_iter(&x[2]).map(|x| {
			Rc::new(CSSRule::new(String::from(&x[1]), String::from((&x[2]).trim())))
		}).collect();

		Rc::new(block)
	}).collect()
}