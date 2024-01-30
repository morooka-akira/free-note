use std::sync::{Arc, Mutex};

pub trait ListInterface<T: std::fmt::Debug> {
    fn new() -> Self;
    /* スタックの操作 */
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    /* キューの操作 */
    fn add(&mut self, element: T);
    fn remove(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Arc<Mutex<Node<T>>>>,
}

#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
    size: usize,
}

impl<T: std::fmt::Debug> Default for List<T> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: Default::default(),
            size: Default::default(),
        }
    }
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        Default::default()
    }

    // 先頭に値を追加する
    pub fn push(&mut self, element: T) {
        let new_node = Arc::new(Mutex::new(Node {
            element,
            next: self.head.take(),
        }));
        self.head = Some(Arc::clone(&new_node));

        // NOTE: Stackの場合は、最初の値が末尾になるため、sizeが0の場合のみtailを更新する
        if self.size == 0 {
            self.tail = Some(Arc::clone(&new_node));
        }
        self.size += 1;
    }

    // 先頭から値を取り出す
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        // NOTE: 先に self.tail を None にしておかないと、headから値を取り出せない(Arc::try_unwrap)
        if self.size == 1 {
            self.tail = None;
        }
        match self.head.take() {
            None => None,
            Some(old_head) => {
                // NOTE: このcloneは、Option::cloneだが、Optionの中身はArcなので、Arc::cloneと同じ挙動をする
                self.head = old_head.lock().unwrap().next.clone();

                self.size -= 1;
                match Arc::try_unwrap(old_head) {
                    Ok(node) => Some(node.into_inner().unwrap().element),
                    Err(_arc) => None,
                }
            }
        }
    }

    // 末尾に値を追加する
    pub fn add(&mut self, element: T) {
        let new_node = Arc::new(Mutex::new(Node {
            element,
            next: None,
        }));

        match self.tail.take() {
            // NOTE: tailに値がない = リストの要素が0
            None => {
                self.head = Some(Arc::clone(&new_node));
                self.tail = Some(Arc::clone(&new_node));
            }
            Some(old_tail) => {
                // NOTE: mutexを使わない場合、Arcの値を書き換えることができない
                old_tail.lock().unwrap().next = Some(Arc::clone(&new_node));
                self.tail = Some(Arc::clone(&new_node));
            }
        }
        self.size += 1;
    }

    // 先頭から値を取り出す
    pub fn remove(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        if self.size == 1 {
            self.tail = None;
        }
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.lock().unwrap().next.clone();

                self.size -= 1;
                match Arc::try_unwrap(old_head) {
                    Ok(node) => Some(node.into_inner().unwrap().element),
                    Err(_arc) => None,
                }
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.pop();
    list.pop();
    list.add(3);
    list.add(4);
    list.remove();
    list.remove();
    println!("{:?}", list);
}
