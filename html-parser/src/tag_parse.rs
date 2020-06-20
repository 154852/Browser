use std::collections::HashMap;

#[derive(Debug)]
pub struct HTMLStartTag {
	pub name: String,
	pub self_close: bool,
	pub attrs: HashMap<String, String>
}
impl HTMLStartTag {
	pub fn new(tagname: String, self_close: bool) -> HTMLStartTag {
		HTMLStartTag {
			name: tagname,
			self_close,
			attrs: HashMap::new()
		}
	}

	pub fn new_with_attrs(tagname: String, self_close: bool, attrs: HashMap<String, String>) -> HTMLStartTag {
		HTMLStartTag {
			name: tagname,
			self_close,
			attrs
		}
	}

	pub fn clone_name(&self) -> String {
		return self.name.clone();
	}

	pub fn clone_attrs(&self) -> HashMap<String, String> {
		return self.attrs.clone();
	}

	pub fn is_self_close(&self) -> bool {
		return self.self_close;
	}
}

#[derive(Debug)]
pub struct HTMLEndTag {}
impl HTMLEndTag {
	pub fn new() -> HTMLEndTag {
		HTMLEndTag {}
	}
}

#[derive(Debug)]
pub enum HTMLChild {
	StartTag(HTMLStartTag),
	EndTag(HTMLEndTag),
	Text(String)
}

pub struct TagParser {
	state: Option<Box<dyn ParseState>>,
	nodes: Vec<HTMLChild>
}
impl TagParser {
	pub fn new() -> TagParser {
		TagParser {
			state: Some(Box::new(InitialState {})),
			nodes: vec![]
		}
	}

	pub fn get_nodes(self) -> Vec<HTMLChild> {
		return self.nodes
	}

	fn emit(&mut self, node: HTMLChild) {
		self.nodes.push(node);
	}

	fn consume(&mut self, ch: char) {
		if let Some(mut state) = self.state.take() {
			state.consume(ch, self);
			self.state = Some(state.next(ch));
		}
	}

	pub fn parse(&mut self, html: &str) {
		for ch in html.bytes() {
			self.consume(ch as char);
		}
	}
}

#[allow(unused_variables)]
trait ParseState {
	fn consume(&mut self, ch: char, parser: &mut TagParser) {}
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState>;
}

struct InitialState {}
impl ParseState for InitialState {
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		match ch {
			'<' => Box::new(TagNameState::new()),
			_ => Box::new(TextNodeState::new(ch.to_string()))
		}
	}
}

struct TextNodeState {
	content: Option<String>
}
impl TextNodeState {
	fn new(content: String) -> TextNodeState {
		TextNodeState {
			content: Some(content)
		}
	}
}
impl ParseState for TextNodeState {
	fn consume(&mut self, ch: char, parser: &mut TagParser) {
		if ch == '<' {
			if let Some(s) = self.content.take() {
				parser.emit(HTMLChild::Text(s));
			} else { panic!("Unreachable") }
		} else {
			if let Some(s) = self.content.as_mut() {
				s.push(ch);
			} else { panic!("Unreachable") }
		}
	}
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		if ch == '<' {
			Box::new(TagNameState::new())
		} else {			
			self
		}
	}
}

struct TagNameState {
	name: Option<String>,
	self_closing: bool,
	is_end: bool
}
impl TagNameState {
	fn new() -> TagNameState {
		TagNameState {
			name: Some(String::new()), self_closing: false, is_end: false
		}
	}

	fn from(name: String, self_closing: bool, is_end: bool) -> TagNameState {
		TagNameState {
			name: Some(name), self_closing, is_end
		}
	}
}
impl ParseState for TagNameState {
	fn consume(&mut self, ch: char, parser: &mut TagParser) {
		match ch {
			' ' => {},
			'/' => {},
			'>' => {
				if self.is_end {
					parser.emit(HTMLChild::EndTag(HTMLEndTag::new()));
				} else if let Some(name) = self.name.take() {
					parser.emit(HTMLChild::StartTag(HTMLStartTag::new(name, self.self_closing)));
				} else { panic!("Unreachable") }
			},
			_ => {
				if let Some(name) = self.name.as_mut() {
					name.push(ch);
				} else { panic!("Unreachable") }
			}
		}
	}

	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		match ch {
			' ' => {
				if let Some(name) = &self.name {
					if name.len() == 0 || self.is_end { return self; }
				}

				if let Some(name) = self.name {
					return Box::new(TagBodyState::new(name, self.self_closing, HashMap::new()));
				}
				
				panic!("Unreachable");
			},
			'/' => {
				if let Some(name) = self.name {
					if name.len() == 0 {
						Box::new(TagNameState::from(name, self.self_closing, true))
					} else {
						Box::new(TagNameState::from(name, true, self.is_end))
					}
				} else {
					panic!("Unreachable");
				}
			},
			'>' => {
				Box::new(InitialState {})
			},
			_ => {
				self
			}
		}
	}
}

struct TagBodyState {
	tagstart: Option<HTMLStartTag>
}
impl TagBodyState {
	fn new(tagname: String, self_closes: bool, attrs: HashMap<String, String>) -> TagBodyState {
		TagBodyState {
			tagstart: Some(HTMLStartTag::new_with_attrs(tagname, self_closes, attrs))
		}
	}

	fn new_from_tag(tag: HTMLStartTag) -> TagBodyState {
		TagBodyState {
			tagstart: Some(tag)
		}
	}
}
impl ParseState for TagBodyState {
	fn consume(&mut self, ch: char, parser: &mut TagParser) {
		match ch {
			'>' => {
				if let Some(tag) = self.tagstart.take() {
					parser.emit(HTMLChild::StartTag(tag));
				} else { panic!("Unreachable"); }
			},
			'/' => {
				if let Some(tag) = self.tagstart.as_mut() {
					tag.self_close = true;
				} else { panic!("Unreachable"); }
			},
			_ => { }
		}
	}
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		match ch {
			'>' => { Box::new(InitialState {}) },
			' ' => { self },
			_ => {
				if let Some(tag) = self.tagstart {
					Box::new(AttributeNameState::new(tag, ch.to_string()))
				} else { panic!("Unreachable") }
			}
		}
	}
}

struct AttributeNameState {
	tag: Option<HTMLStartTag>,
	name: Option<String>
}
impl AttributeNameState {
	fn new(tag: HTMLStartTag, name: String) -> AttributeNameState {
		AttributeNameState {
			name: Some(name), tag: Some(tag)
		}
	}
}
impl ParseState for AttributeNameState {
	fn consume(&mut self, ch: char, _parser: &mut TagParser) {
		if ch != '=' {
			if let Some(name) = self.name.as_mut() {
				name.push(ch);
			} else { panic!("Unreachable") }
		}

	}
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		if ch == '=' {
			if let Some(name) = self.name {
				if let Some(tag) = self.tag {
					return Box::new(AttributeValueState::new(name, String::new(), tag, false));
				}
			}

			panic!("Unreachable!");
		} else {
			self
		}
	}
}

struct AttributeValueState {
	name: Option<String>,
	value: Option<String>,
	tag: Option<HTMLStartTag>,
	has_quotation: bool
}
impl AttributeValueState {
	fn new(name: String, value: String, tag: HTMLStartTag, has_quotation: bool) -> AttributeValueState {
		AttributeValueState {
			name: Some(name), value: Some(value), tag: Some(tag), has_quotation
		}
	}

	fn bind(&mut self, _parser: &mut TagParser) {
		if let Some(mut tag) = self.tag.take() {
			if let Some(name) = self.name.take() { if let Some(value) = self.value.take() {
				tag.attrs.insert(name, value);
			}} else { panic!("Unreachable"); }
			self.tag = Some(tag);
		} else { panic!("Unreachable"); }
	}

	fn bind_and_emit(&mut self, parser: &mut TagParser) {
		if let Some(mut tag) = self.tag.take() {
			if let Some(name) = self.name.take() { if let Some(value) = self.value.take() {
				tag.attrs.insert(name, value);
			}} else { panic!("Unreachable"); }
			parser.emit(HTMLChild::StartTag(tag));
		} else { panic!("Unreachable"); }
	}
}
impl ParseState for AttributeValueState {
	fn consume(&mut self, ch: char, parser: &mut TagParser) {
		match ch {
			'>' => self.bind_and_emit(parser),
			'"' if !self.has_quotation => self.has_quotation = true,
			'"' if self.has_quotation => {
				self.bind(parser);
				self.has_quotation = false;
			},
			_ => {
				if let Some(value) = self.value.as_mut() {
					value.push(ch);
				} else { panic!("Unreachable") }
			}
		}
	}
	fn next(self: Box<Self>, ch: char) -> Box<dyn ParseState> {
		match ch {
			'>' => Box::new(InitialState {}),
			'"' if self.has_quotation => {
				self
			},
			'"' if !self.has_quotation => {
				if let Some(tag) = self.tag {
					Box::new(TagBodyState::new_from_tag(tag))
				} else { panic!("Unreachable"); }
			},
			_ => {
				self
			}
		}
	}
}