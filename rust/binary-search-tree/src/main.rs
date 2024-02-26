use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

trait Set<T> {
    fn add(&self, x: T) -> bool;
    fn remove(&mut self, x: T);
    fn find(&self, x: T) -> Option<T>;
}

#[derive(Debug)]
struct Node<T> {
    left: Option<Arc<Mutex<Node<T>>>>,
    right: Option<Arc<Mutex<Node<T>>>>,
    parent: Option<Arc<Mutex<Node<T>>>>,
    value: Option<T>,
}

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Arc<Mutex<Node<T>>>,
}

impl<T> Set<T> for BinarySearchTree<T>
where
    T: Debug + Eq + PartialOrd + Copy,
{
    fn add(&self, x: T) -> bool {
        let last = self.find_last(x);
        match last {
            Some(last_node) => {
                if last_node.lock().unwrap().value == Some(x) {
                    false
                } else {
                    let new_node = Arc::new(Mutex::new(Node {
                        left: None,
                        right: None,
                        parent: None,
                        value: Some(x),
                    }));
                    self.add_child(last_node, new_node);
                    true
                }
            }
            None => false,
        }
    }

    fn remove(&mut self, x: T) {
        if let Some(node) = self.find_node(x) {
            if node.lock().unwrap().left.is_none() || node.lock().unwrap().right.is_none() {
                self.splice(Arc::clone(&node));
            } else {
                let current = Arc::clone(&node);
                let mut next = current.lock().unwrap().right.as_ref().map(Arc::clone);
                while let Some(n) = &next {
                    let left_option = n.lock().unwrap().left.as_ref().map(Arc::clone);
                    if left_option.is_some() {
                        next = left_option;
                    } else {
                        break;
                    }
                }
                if let Some(n) = next {
                    current.lock().unwrap().value = n.lock().unwrap().value;
                    println!("before splice... ");
                    self.splice(Arc::clone(&n));
                    println!("splice... ");
                }
            }
        }
    }

    fn find(&self, x: T) -> Option<T> {
        if let Some(node) = self.find_node(x) {
            node.lock().unwrap().value
        } else {
            None
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Debug + Eq + PartialOrd + Copy,
{
    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Arc::new(Mutex::new(Node {
                left: None,
                right: None,
                parent: None,
                value: None,
            })),
        }
    }

    fn depth(&self, node: Arc<Mutex<Node<T>>>) -> u32 {
        if Arc::ptr_eq(&node, &self.root) {
            0
        } else {
            1 + self.depth(Arc::clone(&node))
        }
    }

    fn find_last(&self, x: T) -> Option<Arc<Mutex<Node<T>>>> {
        let mut current = Arc::clone(&self.root);
        loop {
            let current_value = current.lock().unwrap().value;
            match current_value {
                Some(ref value) if value == &x => return Some(Arc::clone(&current)),
                Some(ref value) if x < *value => {
                    let next = { current.lock().unwrap().left.as_ref().map(Arc::clone) };
                    if let Some(next_node) = next {
                        current = next_node;
                    } else {
                        return Some(Arc::clone(&current));
                    }
                }
                Some(_) => {
                    let next = { current.lock().unwrap().right.as_ref().map(Arc::clone) };
                    if let Some(next_node) = next {
                        current = next_node;
                    } else {
                        return Some(Arc::clone(&current));
                    }
                }
                None => return Some(Arc::clone(&current)),
            }
        }
    }

    fn add_child(&self, parent: Arc<Mutex<Node<T>>>, child: Arc<Mutex<Node<T>>>) {
        let mut parent_lock = parent.lock().unwrap();
        let mut child_lock = child.lock().unwrap();
        if parent_lock.value.is_none() {
            parent_lock.value = child_lock.value;
            return;
        }
        if child_lock.value.unwrap() < parent_lock.value.unwrap() {
            parent_lock.left = Some(Arc::clone(&child));
        } else {
            parent_lock.right = Some(Arc::clone(&child));
        }
        child_lock.parent = Some(Arc::clone(&parent));
    }

    fn splice(&mut self, node: Arc<Mutex<Node<T>>>) {
        let mut child = None;
        if node.lock().unwrap().left.is_some() {
            child = node.lock().unwrap().left.clone();
        } else if node.lock().unwrap().right.is_some() {
            child = node.lock().unwrap().right.clone();
        }

        if let Some(ref c) = child {
            c.lock().unwrap().parent = node.lock().unwrap().parent.clone();
        }

        if let Some(ref parent) = node.lock().unwrap().parent {
            if Arc::ptr_eq(&node, parent.lock().unwrap().left.as_ref().unwrap()) {
                parent.lock().unwrap().left = child;
            } else {
                parent.lock().unwrap().right = child;
            }
        } else {
            self.root = child.unwrap();
        }
    }

    fn find_node(&self, x: T) -> Option<Arc<Mutex<Node<T>>>> {
        let mut current = Arc::clone(&self.root);
        loop {
            let current_value = current.lock().unwrap().value;
            println!("loop... {:?}", current_value);
            match current_value {
                Some(ref value) if value == &x => return Some(Arc::clone(&current)),
                Some(ref value) if x < *value => {
                    println!("next left");
                    let next = { current.lock().unwrap().left.as_ref().map(Arc::clone) };
                    if let Some(next_node) = next {
                        println!("next left");
                        current = next_node;
                    } else {
                        return None;
                    }
                }
                Some(_) => {
                    println!("next left");
                    let next = { current.lock().unwrap().right.as_ref().map(Arc::clone) };
                    if let Some(next_node) = next {
                        println!("next right");
                        current = next_node;
                    } else {
                        return None;
                    }
                }
                None => return None,
            }
        }
    }
}

fn main() {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    tree.add(5);
    tree.add(3);
    tree.add(7);
    tree.add(2);
    tree.add(4);
    tree.add(6);
    tree.add(8);

    println!("{:?}", tree.find(5));

    tree.remove(5);

    println!("{:?}", tree.find(5));

    println!("{:?}", tree);
}
