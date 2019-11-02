
use std::rc::Rc; // shared pointer
use std::cell::RefCell; // mutability 

type NodePtr<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    value: T,
    prev: Option<NodePtr<T>>,
    next: Option<NodePtr<T>>
}


pub struct DoublyLinkedList<T> {
    first: Option<NodePtr<T>>,
    last: Option<NodePtr<T>>,
    curr_iter: Option<NodePtr<T>>
}

impl<T> Node<T> {
    pub fn new_ptr(new_value: T) -> NodePtr<T> {
        return Rc::new(RefCell::new(Node {
            value: new_value,
            prev: None,
            next: None,
        }))
    }

    pub fn set_next(&mut self, last: Option<NodePtr<T>>)  {
        self.next = last;
    }
    
    pub fn set_prev(&mut self, prev: Option<NodePtr<T>>)  {
        self.prev = prev;
    }

    pub fn get_next(&self) -> Option<NodePtr<T>> {
        if let Some(next_node) = &self.next {
            return Some(next_node.clone())
        }
        None
    }
}

impl<T> DoublyLinkedList<T> 
where T: Copy, T: PartialEq
{
    pub fn new() -> DoublyLinkedList<T>{
        return DoublyLinkedList {
            first: None,
            last: None,
            curr_iter: None
        }
    }

    pub fn insert(&mut self, new_value: T) {
        if let Some(prev_last_ptr) = &self.last {
            let new_last_ptr = Node::new_ptr(new_value);
            prev_last_ptr.borrow_mut().set_next(Some(new_last_ptr.clone()));
            new_last_ptr.borrow_mut().set_prev(Some(prev_last_ptr.clone()));
            self.last = Some(new_last_ptr);
            return;
        }
        let node_ptr = Node::new_ptr(new_value);
        self.first = Some(node_ptr.clone());
        self.last = Some(node_ptr.clone());
        self.curr_iter = Some(node_ptr.clone());
    } 

    fn find(&self, value: T) -> Option<NodePtr<T>>{
        let mut next_node_option = None;
        if let Some(first_node) = &self.first {
            next_node_option = Some(first_node.clone());
        }
        while let Some(next_node) = next_node_option {
            if next_node.borrow().value == value {
                return Some(next_node.clone())
            }
            next_node_option = next_node.borrow().get_next();
        }
        None
    }

    pub fn contains(&self, value: T) -> bool {
        match self.find(value) {
            Some(_) => true,
            None => false,
        }
    } 
}

impl<T> Iterator for DoublyLinkedList<T> 
where T: Copy 
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(curr_iter_ptr) = self.curr_iter.clone() {
            match &curr_iter_ptr.borrow().next {
                    Some(next_iter_ptr) => {
                        self.curr_iter = Some(next_iter_ptr.clone());
                        return Some(curr_iter_ptr.borrow().value);
                    }
                    None => {
                        self.curr_iter = None;
                        return Some(curr_iter_ptr.borrow().value);
                    }
                }
        }
        return None;
    }
}

#[test]
fn insert_test() {
    let mut list: DoublyLinkedList<i8> = DoublyLinkedList::new();
    let vector_from_range: Vec<i8> = (1..20).collect();
    let mut vector_from_list: Vec<i8> = Vec::new();

    for i in 1..20 {
        list.insert(i)
    }

    for j in list {
        vector_from_list.push(j)
    }

    assert_eq!(vector_from_range, vector_from_range)
}

#[test]
fn contains_test() {
    let mut list: DoublyLinkedList<i8> = DoublyLinkedList::new();

    for i in 1..20 {
        list.insert(i)
    }

    assert_eq!(list.contains(3), true);
    assert_eq!(list.contains(19), true);
    assert_ne!(list.contains(20), true);
    assert_eq!(list.contains(21), false);
}

fn main () {
    
}
