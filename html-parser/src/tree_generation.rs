use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::tag_parse::*;

#[derive(Debug)]
pub struct HTMLRoot<R> {
    pub children: RefCell<Vec<Rc<HTMLNode<R>>>>
}
impl<R> HTMLRoot<R> {
    pub fn new() -> HTMLRoot<R> {
        HTMLRoot {
            children: RefCell::new(vec![])
        }
    }
}

#[derive(Debug)]
pub struct HTMLElement<R> {
    pub tagname: String,
    pub attrs: HashMap<String, String>,
    pub children: RefCell<Vec<Rc<HTMLNode<R>>>>,
    pub parent: RefCell<Weak<HTMLNode<R>>>
}
impl<R> HTMLElement<R> {
    fn new(start_tag: &HTMLStartTag, parent: RefCell<Weak<HTMLNode<R>>>) -> HTMLElement<R> {
        HTMLElement {
            tagname: start_tag.clone_name(),
            attrs: start_tag.clone_attrs(),
            children: RefCell::new(vec![]),
            parent
        }
    }
    
    pub fn first_child(&self) -> Option<Rc<HTMLNode<R>>> {
        let children = self.children.borrow();
        let children: &Vec<Rc<HTMLNode<R>>> = children.as_ref();
        println!("{:?}", children.len());
        if let Some(child) = children.get(0) { Some(child.clone()) } else { None }
    }
}

#[derive(Debug)]
pub struct HTMLText<R> {
    pub content: String,
    pub parent: RefCell<Weak<HTMLNode<R>>>
}
impl<R> HTMLText<R> {
    fn new(content: String, parent: RefCell<Weak<HTMLNode<R>>>) -> HTMLText<R> {
        HTMLText {
            content,
            parent
        }
    }
}

#[derive(Debug)]
pub enum HTMLNodeContent<R> {
    Text(Rc<HTMLText<R>>),
    Element(Rc<HTMLElement<R>>),
    Root(Rc<HTMLRoot<R>>)
}

#[derive(Debug)]
pub struct HTMLNode<R> {
    pub content: Rc<HTMLNodeContent<R>>,
    pub renderer: R
}
impl<R> HTMLNode<R> {
    fn new(content: HTMLNodeContent<R>, generator: &mut impl RenderGenerator<R>) -> HTMLNode<R> {
        let rc = Rc::new(content);
        HTMLNode {
            renderer: generator.generate(rc.clone()),
            content: rc
        }
    }
}

pub trait RenderGenerator<T> {
    fn generate(&mut self, node: Rc<HTMLNodeContent<T>>) -> T;
}

pub fn create_elements<R>(tags: Vec<HTMLChild>, generator: &mut impl RenderGenerator<R>) -> Rc<HTMLNode<R>> {
    let mut parent = Rc::new(HTMLNode::new(HTMLNodeContent::Root(Rc::new(HTMLRoot::new())), generator));
    let root = Rc::clone(&parent);

    for tag in tags {
        match tag {
            HTMLChild::StartTag(tag) => {
                let core_element = Rc::new(HTMLElement::new(&tag, RefCell::new(Rc::downgrade(&parent))));
                let element = Rc::new(HTMLNode::new(HTMLNodeContent::Element(core_element.clone()), generator));

                if let HTMLNodeContent::Root(p) = &parent.as_ref().content.as_ref() {
                    p.children.borrow_mut().push(element.clone());
                } else if let HTMLNodeContent::Element(p) = &parent.as_ref().content.as_ref() {
                    p.children.borrow_mut().push(element.clone());
                } else { panic!("Unreachable") }

                if !tag.is_self_close() { parent = element; }
            },
            HTMLChild::EndTag(_) => {
                let mut new_parent: Option<Rc<HTMLNode<R>>> = None;
                if let HTMLNodeContent::Element(p) = &parent.as_ref().content.as_ref() {
                    new_parent = p.parent.borrow_mut().upgrade();
                }

                if let Some(np) = new_parent {
                    parent = np;
                }
            },
            HTMLChild::Text(string) => {
                let core_element = Rc::new(HTMLText::new(string, RefCell::new(Rc::downgrade(&parent))));
                let element = Rc::new(HTMLNode::new(HTMLNodeContent::Text(core_element.clone()), generator));

                if let HTMLNodeContent::Root(p) = &parent.as_ref().content.as_ref() {
                    p.children.borrow_mut().push(element.clone());
                } else if let HTMLNodeContent::Element(p) = &parent.as_ref().content.as_ref() {
                    p.children.borrow_mut().push(element.clone());
                } else { panic!("Unreachable") }
            }
        }
    }

    root
}