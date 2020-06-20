use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TreeNode<T> {
    parent: RefCell<Weak<TreeNode<T>>>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
    pub value: RefCell<T>
}

impl<T> TreeNode<T> {
    pub fn new(value: T, parent: Rc<TreeNode<T>>) -> Rc<TreeNode<T>> {
        Rc::new(TreeNode {
            parent: RefCell::new(Rc::downgrade(&parent)),
            children: RefCell::new(vec![]),
            value: RefCell::new(value)
        })
    }

    pub fn root(value: T) -> Rc<TreeNode<T>> {
        Rc::new(TreeNode {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            value: RefCell::new(value)
        })
    }

    pub fn add_child(self: Rc<Self>, child: Rc<TreeNode<T>>) {
        self.children.borrow_mut().push(child);
    }

    pub fn get_child(&self, idx: usize) -> Option<Rc<TreeNode<T>>> {
        if let Some(r) = self.children.borrow().get(idx) { Some(r.clone()) } else { None }
    }

    pub fn get_parent(&self) -> Option<Rc<TreeNode<T>>> {
        self.parent.borrow_mut().upgrade()
    }

    fn _map<A, B>(self: Rc<Self>, cb: &A, parent: Rc<TreeNode<B>>) -> Rc<TreeNode<B>> where A: Fn(Rc<TreeNode<T>>) -> B {
        let root = TreeNode::new(cb(self.clone()), parent);
        
        for child in self.children.borrow().iter() {
            root.clone().add_child(child.clone()._map(cb, root.clone()));
        }

        root
    }

    pub fn map<A, B>(self: Rc<Self>, cb: A) -> Rc<TreeNode<B>> where A: Fn(Rc<TreeNode<T>>) -> B {
        let root = TreeNode::root(cb(self.clone()));

        for child in self.children.borrow().iter() {
            root.clone().add_child(child.clone()._map(&cb, root.clone()));
        }

        root
    }

    fn _find<A>(self: Rc<Self>, cb: &A, matches: &mut Vec<Rc<TreeNode<T>>>) where A: Fn(Rc<TreeNode<T>>) -> bool {
        if cb(self.clone()) { matches.push(self.clone()); }
        
        for child in self.children.borrow().iter() {
            child.clone()._find(cb, matches);
        }
    }

    pub fn find<A>(self: Rc<Self>, cb: A) -> Vec<Rc<TreeNode<T>>> where A: Fn(Rc<TreeNode<T>>) -> bool {
        let mut matches = Vec::new();
        self._find(&cb, &mut matches);
        matches
    }

    fn _map_linear<A, B>(self: Rc<Self>, cb: &A, matches: &mut Vec<B>) where A: Fn(Rc<TreeNode<T>>) -> B {
        matches.push(cb(self.clone()));
        
        for child in self.children.borrow().iter() {
            child.clone()._map_linear(cb, matches);
        }
    }

    pub fn map_linear<A, B>(self: Rc<Self>, cb: A) -> Vec<B> where A: Fn(Rc<TreeNode<T>>) -> B {
        let mut matches = Vec::new();
        self._map_linear(&cb, &mut matches);
        matches
    }

    fn _walk<A>(self: Rc<Self>, cb: &A) where A: Fn(Rc<TreeNode<T>>) {
        cb(self.clone());
        
        for child in self.children.borrow().iter() {
            child.clone()._walk(cb);
        }
    }

    pub fn walk<A>(self: Rc<Self>, cb: A) where A: Fn(Rc<TreeNode<T>>) {
        self._walk(&cb);
    }
}