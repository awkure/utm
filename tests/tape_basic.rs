//! `tape_basic.rs`
//!
//! This module contains a basic test for the tape.
//!
//! Status: Finished | Unrefactored | Released

extern crate utm;

use utm::tape::{Tape, TapeBasics};

/// Function        basics
/// Describtion     Just a basic test for the tape
#[test]
fn basics() {
    /// Make a blank tape
    let mut tape = Tape::new();

    /// Check empty tape behaves right
    assert_eq!(tape.pop_front(), None);

    /// Populate the tape
    tape.push_front(1);
    tape.push_front(2);
    tape.push_front(3);

    /// Check normal removal
    assert_eq!(tape.pop_front(), Some(3));
    assert_eq!(tape.pop_front(), Some(2));

    /// Push some more just to make sure nothing's corrupted
    tape.push_front(4);
    tape.push_front(5);

    /// Check normal removal
    assert_eq!(tape.pop_front(), Some(5));
    assert_eq!(tape.pop_front(), Some(4));

    /// Check exhaustion
    assert_eq!(tape.pop_front(), Some(1));
    assert_eq!(tape.pop_front(), None);

    /// Check empty tape behaves right
    assert_eq!(tape.pop_back(), None);

    /// Populate the tape
    tape.push_back(1);
    tape.push_back(2);
    tape.push_back(3);

    /// Check normal removal
    assert_eq!(tape.pop_back(), Some(3));
    assert_eq!(tape.pop_back(), Some(2));

    /// Push some more just to make sure nothing's corrupted
    tape.push_back(4);
    tape.push_back(5);

    /// Check normal removal
    assert_eq!(tape.pop_back(), Some(5));
    assert_eq!(tape.pop_back(), Some(4));

    /// Check exhaustion
    assert_eq!(tape.pop_back(), Some(1));
    assert_eq!(tape.pop_back(), None);
}

#[test]
fn peeking() {
    /// Make a blank tape
    let mut tape = Tape::new();

    /// Check empty tape behaves right
    assert!(tape.peek_front().is_none());
    assert!(tape.peek_back().is_none());
    assert!(tape.peek_front_mut().is_none());
    assert!(tape.peek_back_mut().is_none());

    /// Populate the tape
    tape.push_front(1); 
    tape.push_front(2); 
    tape.push_front(3);

    /// Check normal peeking
    assert_eq!(&*tape.peek_front().unwrap(), &3);
    assert_eq!(&*tape.peek_back().unwrap(),  &1);
    assert_eq!(&mut *tape.peek_front_mut().unwrap(), &mut 3);
    assert_eq!(&mut *tape.peek_back_mut().unwrap(),  &mut 1);
}

#[test]
fn into_iter() {
    /// Make a blank tape
    let mut tape = Tape::new();

    /// Populate the tape
    tape.push_front(1); 
    tape.push_front(2); 
    tape.push_front(3);

    /// Make an iterator over the tape
    let mut iter = tape.into_iter();

    /// Check normal iteration
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn moving() {
    /// Make a blank tape
    let mut tape = Tape::new();

    /// Check empty tape behaves right
    tape.move_left();
    assert!(tape.peek_front().is_none());
    assert!(tape.peek_back().is_none());
    assert!(tape.peek_front_mut().is_none());
    assert!(tape.peek_back_mut().is_none());

    /// Check empty tape behaves right, again
    tape.move_right();
    assert!(tape.peek_front().is_none());
    assert!(tape.peek_back().is_none());
    assert!(tape.peek_front_mut().is_none());
    assert!(tape.peek_back_mut().is_none());


    /// Populate the tape
    /// tape :: [ 3 | 2 | 1 ]
    tape.push_front(1); tape.push_front(2); tape.push_front(3);
        
    /// Check moving 
    /// tape :: [ 1 | 3 | 2 ]
    tape.move_right();
    assert_eq!(&*tape.peek_front().unwrap(), &1);
    assert_eq!(&*tape.peek_back().unwrap(),  &2);
    assert_eq!(&mut *tape.peek_front_mut().unwrap(), &mut 1);
    assert_eq!(&mut *tape.peek_back_mut().unwrap(),  &mut 2);
    
    /// Check moving, again
    /// tape :: [ 3 | 2 | 1 ]
    tape.move_left();
    assert_eq!(&*tape.peek_front().unwrap(), &3);
    assert_eq!(&*tape.peek_back().unwrap(),  &1);
    assert_eq!(&mut *tape.peek_front_mut().unwrap(), &mut 3);
    assert_eq!(&mut *tape.peek_back_mut().unwrap(),  &mut 1);
}
        
