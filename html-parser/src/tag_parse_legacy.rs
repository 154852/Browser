#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct HTMLStartTag {
    name: String,
    attrs: HashMap<String, String>,
    self_closes: bool
}

impl HTMLStartTag {
    fn new(name: &str) -> HTMLStartTag {
        HTMLStartTag {
            name: String::from(name),
            attrs: HashMap::new(),
            self_closes: false
        }
	}
	
	pub fn clone_name(&self) -> String {
		self.name.clone()
	}

	pub fn clone_attrs(&self) -> HashMap<String, String> {
		self.attrs.clone()
	}

	pub fn is_closed(&self) -> bool {
		self.self_closes
	}
}

#[derive(Debug)]
pub struct HTMLEndTag {
    name: String
}

impl HTMLEndTag {
    fn new(name: &str) -> HTMLEndTag {
        HTMLEndTag {
            name: String::from(name),
        }
    }
}

enum ParserState {
    Initial,
    TagOpen,
    TagOpenName,
    StartTagAttrName,
    StartTagAttrContent
}

#[derive(Debug)]
pub enum HTMLChild {
    StartTag(HTMLStartTag),
    EndTag(HTMLEndTag),
    Text(String)
}

pub fn parse_tags(html: &str) -> Vec<Rc<HTMLChild>> {
    let mut state = ParserState::Initial;
    let mut children: Vec<Rc<HTMLChild>> = Vec::new();
    let mut working_child: Option<Rc<HTMLChild>> = None;
    let mut working_attribute: Option<(String, String)> = None;
    let mut attribute_has_quotation = false;

    let emit = |children: &mut Vec<Rc<HTMLChild>>, state: &mut ParserState, working_child: &mut Option<Rc<HTMLChild>>| {
        if let Some(child) = &working_child {
            children.push(Rc::clone(&child));
            *working_child = None;
            *state = ParserState::Initial;
        }
    };

    for (_i, charcode) in html.as_bytes().iter().enumerate() {
        let html_char = *charcode as char;
        match state {
            ParserState::Initial => match html_char {
                '<' => {
                    state = ParserState::TagOpenName;
                    if let Some(child) = &working_child { children.push(Rc::clone(&child)); }
                    working_child = Some(Rc::new(HTMLChild::StartTag(HTMLStartTag::new(""))));
                },
                _ => {
                    if let Some(child) = &mut working_child {
                        match Rc::get_mut(child).unwrap() {
                            HTMLChild::StartTag(_) => panic!("Unreachable"),
                            HTMLChild::EndTag(_) => panic!("Unreachable"),
                            HTMLChild::Text(text) => text.push(html_char),
                        }
                    } else {
                        working_child = Some(Rc::new(HTMLChild::Text(html_char.to_string())));
                    }
                }
            },
            ParserState::TagOpenName => match html_char {
                ' ' => {
                    if let Some(child) = &working_child {
                        if let HTMLChild::StartTag(_) = child.as_ref() {
                            state = ParserState::TagOpen;
                        }
                    }
                },
                '>' => emit(&mut children, &mut state, &mut working_child),
                '/' => working_child = Some(Rc::new(HTMLChild::EndTag(HTMLEndTag::new("")))),
                _ => {
                    if let Some(child) = &mut working_child {
                        match Rc::get_mut(child).unwrap() {
                            HTMLChild::StartTag(tag) => tag.name.push(html_char),
                            HTMLChild::EndTag(tag) => tag.name.push(html_char),
                            HTMLChild::Text(_) => panic!("Unreachable"),
                        }
                    }
                }
            },
            ParserState::TagOpen => match html_char {
                '>' => emit(&mut children, &mut state, &mut working_child),
                '/' => {
                    if let Some(child) = &mut working_child {
                        match Rc::get_mut(child).unwrap() {
                            HTMLChild::StartTag(tag) => tag.self_closes = true,
                            HTMLChild::EndTag(_) => panic!("Unreachable"),
                            HTMLChild::Text(_) => panic!("Unreachable"),
                        }
                    }
                },
                ' ' => {},
                _ => {
                    state = ParserState::StartTagAttrName;
                    working_attribute = Some((html_char.to_string(), String::new()));
                }
            }
            ParserState::StartTagAttrName => match html_char {
                '=' => state = ParserState::StartTagAttrContent,
                _ => {
                    if let Some(attr) = &mut working_attribute {
                        attr.0.push(html_char);
                    }
                }
            }
            ParserState::StartTagAttrContent => match html_char {
                '>' => {
                    if let Some(child) = &mut working_child { if let Some((k, v)) = &working_attribute {
                        match Rc::get_mut(child).unwrap() {
                            HTMLChild::StartTag(tag) => {tag.attrs.insert(k.to_string(), v.to_string());},
                            HTMLChild::EndTag(_) => panic!("Unreachable"),
                            HTMLChild::Text(_) => panic!("Unreachable"),
                        }
                    }}
                    emit(&mut children, &mut state, &mut working_child);
                },
                '"' if !attribute_has_quotation => attribute_has_quotation = true,
                '"' if attribute_has_quotation => {
                    if let Some(child) = &mut working_child { if let Some((k, v)) = &working_attribute {
                        match Rc::get_mut(child).unwrap() {
                            HTMLChild::StartTag(tag) => {tag.attrs.insert(k.to_string(), v.to_string());},
                            HTMLChild::EndTag(_) => panic!("Unreachable"),
                            HTMLChild::Text(_) => panic!("Unreachable"),
                        }
                    }}
                    working_attribute = None;
                    state = ParserState::TagOpen;
                    attribute_has_quotation = false;
                },
                _ => {
                    if let Some(attr) = &mut working_attribute {
                        attr.1.push(html_char);
                    }
                }
            }
        }
	}
	
	if let Some(child) = &working_child { children.push(Rc::clone(&child)); }

    children
}