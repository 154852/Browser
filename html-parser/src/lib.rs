use std::rc::Rc;

pub mod tree_generation_2;
use tree;
mod tag_parse;
pub mod tree_generation;
pub mod css_parse;

pub fn parse_html<R>(html: &str, generator: &mut impl tree_generation::RenderGenerator<R>) -> Rc<tree_generation::HTMLNode<R>> {
    let mut x = tag_parse::TagParser::new();
    x.parse(html);
    tree_generation::create_elements(x.get_nodes(), generator)
}

pub fn parse_html_new(html: &str) -> Rc<tree::TreeNode<tree_generation_2::HTMLNode>> {
    let mut x = tag_parse::TagParser::new();
    x.parse(html);
    tree_generation_2::create_node_tree(x.get_nodes())
}

pub fn parse_css(css: &str) -> Vec<Rc<css_parse::CSSBlock>> {
    css_parse::create_blocks(css)
}

pub fn parse_css_inner(css: &str) -> css_parse::CSSBlock {
    css_parse::create_block(css)
}

pub trait WalkCB<R> {
    fn element(&mut self, el: Rc<tree_generation::HTMLNode<R>>);
}

pub fn walk<R>(root: Rc<tree_generation::HTMLNode<R>>, cb: &mut impl WalkCB<R>) {
    cb.element(root.clone());

    match root.as_ref().content.as_ref() {
        tree_generation::HTMLNodeContent::Root(rt) => {
            for child in rt.children.borrow().iter() {
                walk(child.clone(), cb);
            }
        },
        tree_generation::HTMLNodeContent::Element(el) => {
            for child in el.children.borrow().iter() {
                walk(child.clone(), cb);
            }
        },
        tree_generation::HTMLNodeContent::Text(_) => {}
    }
}