//! `tape.rs`
//!
//! This module contains tape implementation
//! as a doubly linked almost thread safe queue
//!
//! Status: Finished | Refactored | Released

use ::std::sync::Arc;

use ::std::cell::{
    Ref, 
    RefMut, 
    RefCell
};

/// The tape struct itself where ir has
/// 2 links of some messy field threaded field.
#[derive(Debug, Clone)]
pub struct Tape<T> {
    head: Link<Node<T>>,
    tail: Link<Node<T>>,
}

/// Need for an iteration over the tape
#[derive(Clone)]
pub struct IntoIter<T>(Tape<T>);

/// Pointer to a specific type, needed for concurrency
pub type Link<T> = Option<Arc<RefCell<T>>>;


/// The node of the tape itself
#[derive(Debug, Clone)]
struct Node<T> {
    elem: T,
    next: Link<Node<T>>,
    prev: Link<Node<T>>,
}

/// I've used it as separate trate for debubugging and pretify purpose only
pub trait TapeBasics<T> {
    fn new() -> Self;
    fn     push_front(&mut self, elem: T);
    fn      push_back(&mut self, elem: T);
    fn      pop_front(&mut self) -> Option<T>;
    fn       pop_back(&mut self) -> Option<T>;
    fn peek_front_mut(&mut self) -> Option<RefMut<T>>;
    fn  peek_back_mut(&mut self) -> Option<RefMut<T>>;
    fn         peek_front(&self) -> Option<Ref<T>>;
    fn          peek_back(&self) -> Option<Ref<T>>;
    fn           into_iter(self) -> IntoIter<T>;
    fn     move_right(&mut self);
    fn      move_left(&mut self);
}

/// A little implementation for <Node<T>> 
/// where you need no more than a blank function constructor 
impl<T> Node<T> {
    
    /// Function        new
    /// Description     Makes new blank node with the given element inside
    ///
    /// elem            The given element described above
    ///
    /// return          Arc<RefCell<T>>  
    fn new(elem: T) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

/// Where the heart of the tape goes
impl<T> TapeBasics<T> for Tape<T> {
    
    /// Function        new
    /// Purpose         Makes a new blank tape
    ///
    /// return          Tape<Option<T>>
    fn new() -> Self {
        Tape { head: None, tail: None }
    }


    /// Function        push_front
    /// Purpose         Pushes the given element to the front of the tape
    ///
    /// &mut self       The given tape as a method
    /// elem            The given element you want to push
    ///
    /// return          ()
    fn push_front(&mut self, elem: T) {
        let new_head: Arc<RefCell<Node<T>>> = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }


    /// Function        push_back
    /// Purpose         Pushes the given element to the back of the tape
    ///
    /// &mut self       The given tape as a method
    /// elem            The given element you want to push
    ///
    /// return          ()
    fn push_back(&mut self, elem: T) {
        let new_tail: Arc<RefCell<Node<T>>> = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }


    /// Function        pop_front
    /// Purpose         Pops the head (front/first element) 
    ///                 of the tape and returns it
    ///
    /// &mut self       The given tape you want to modify
    ///
    /// return          Option<T>
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Arc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }


    /// Function        pop_back
    /// Purpose         Pops the last element 
    ///                 (tail of the pre-last node) and returns it
    ///
    /// &mut self       The given tape you want to modify
    ///
    /// return          Option<T>      
    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Arc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }


    /// Function        pop_front
    /// Purpose         Peeks inside the tape and returns a read only 
    ///                 reference to its front/head element
    ///
    /// &mut self       The given tape you want to peek in 
    ///
    /// return          Option<Ref<T>>
    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }


    /// Function        peek_front_mut
    /// Purpose         Peeks inside the tape and returns a mutable
    ///                 reference to its front/head element
    ///
    /// &mut self       The given tape you want to peek in
    ///
    /// return          Option<RefMut<T>>
    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }


    /// Function        peek_back
    /// Purpose         Peeks inside the tape and returns a read only 
    ///                 reference to its back/(last element) 
    ///
    /// &mut self       The given tape you want to peek in
    ///
    /// return          Option<Ref<T>>
    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }


    /// Function        peek_back_mut
    /// Purpose         Peeks inside the tape and returns a mutable 
    ///                 reference to its back(last element)
    ///
    /// &mut self       The given tape you want to peek in
    ///
    /// return          Option<RefMut<T>>
    fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
    

    /// Define vulnerabilities here 
    /// Function        move_right 
    /// Purpose         'Moves' the tape to the right 
    ///                 (pops its last element and pushes it to the front)
    ///
    /// &mut self       The given tape you want to modify
    ///
    /// return          ()
    fn move_right(&mut self) {
        self.pop_back().map(|node| { self.push_front(node); });
    }


    /// Function        move_left
    /// Purpose         'Moves' the tape to the left
    ///                 (pops its first element and pushes it to the back)
    ///
    /// &mut self       The given tape you want to modify
    ///
    /// return          ()
    fn move_left(&mut self) {
        self.pop_front().map(|node| { self.push_back(node); });
    }


    /// Function        into_iter
    /// Purpose         Makes tha tape iterable over it
    ///
    /// self            The fiven tape you want to make iterable over
    ///
    /// return          IntoIter<T>
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}


/// Specifies how to get rid of the tape while it goes out of the scope
impl<T> Drop for Tape<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() { }
    }
}


/// How iterator should behave when you try to iter over the tape
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}


/// The same as the previous but it starts 
/// over the end of the tape (reverse iteration)
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}


