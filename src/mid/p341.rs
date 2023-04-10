#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

use std::{vec::IntoIter, iter::Peekable};

struct NestedIterator {
    next: Option<Box<NestedIterator>>,
    data: Peekable<IntoIter<NestedInteger>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let iter = nestedList.into_iter();
        return NestedIterator { next: None, data: iter.peekable() }
    }
    
    fn next(&mut self) -> i32 {
        if let Some(iter) = self.next.as_mut() {
            let next = iter.next();
            if !iter.has_next() {
                self.next = None;
            }
            return next;
        } else if let Some(next_int) = self.data.next() {
            match next_int {
                NestedInteger::Int(int) => return int,
                NestedInteger::List(vec) => {
                    let mut new_iter = NestedIterator::new(vec);
                    let next = new_iter.next();
                    self.next = if new_iter.has_next() {Some(Box::new(new_iter))} else {None};
                    return next;
                },
            }
        } else {
            0
        }
    }

    fn skip_empty(&mut self) {
        
    }
    
    pub fn has_next(&mut self) -> bool {
        self.next.is_some() || self.data.peek().is_some()
    }
}