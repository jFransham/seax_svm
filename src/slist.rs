pub use slist::List::{Cons,Nil};

use std::fmt;
use std::ops::Index;
use std::iter::{IntoIterator, FromIterator};

/// Convenience macro for making lists.
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// assert_eq!(
///     list!(1i32, 2i32, 3i32),
///     Cons(1i32, Box::new(Cons(2i32, Box::new(Cons(3i32, Box::new(Nil))))))
///     );
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
macro_rules! list(
    ( $e:expr, $($rest:expr),+ ) => ( Cons($e, Box::new(list!( $( $rest ),+ )) ));
    ( $e:expr ) => ( Cons($e, Box::new(Nil)) );
    () => ( Box::new(Nil) );
);

/// Common functions for an immutable Stack abstract data type.
#[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
pub trait Stack<T>: Sized {

    /// Push an item to the top of the stack, returning a new stack
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn push(self, item : T) -> Self;

    /// Pop the top element of the stack. Returns an Option on a T and
    /// a new Stack<T> to replace this.
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn pop(self)            -> Option<(T, Self)>;

    /// Peek at the top item of the stack.
    ///
    /// Returns Some<T> if there is an item on top of the stack,
    /// and None if the stack is empty.
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn peek(&self)          -> Option<&T>;

    /// Returns an empty stack.
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn empty()              -> Self;
}

/// Stack implementation using a `cons` list
impl<T> Stack<T> for List<T> {

    /// Push an item to the top of the stack, returning a new stack.
    ///
    /// # Examples:
    /// ```
    /// use seax_svm::slist::{List,Stack};
    ///
    /// let mut s: List<isize> = Stack::empty();
    /// assert_eq!(s.peek(), None);
    /// s = s.push(1);
    /// assert_eq!(s.peek(), Some(&1));
    /// s = s.push(6);
    /// assert_eq!(s.peek(), Some(&6));
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn push(self, item: T) -> List<T> { Cons(item, Box::new(self)) }

    /// Pop the top element of the stack.
    ///
    /// Pop the top element of the stack. Returns an
    /// `Option<(T,List<T>)>` containing the top element and a new
    /// `List<T>` with that item removed, or `None` if the stack is
    /// empty.
    ///
    /// # Examples:
    /// ```
    /// # use seax_svm::slist::{List,Stack};
    ///
    /// let mut s: List<isize> = Stack::empty();
    /// s = s.push(2);
    /// s = s.push(1);
    /// let pop_result = s.pop().unwrap();
    /// s = pop_result.1;
    /// assert_eq!(s.peek(), Some(&2));
    /// assert_eq!(pop_result.0, 1);
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn pop(self) -> Option<(T,List<T>)> {
        match self {
            Cons(item, new_self)    => Some((item, *new_self)),
            Nil                     => None
        }
    }

    #[inline]
    #[cfg_attr(feature = "nightly",
        stable(feature="stack", since="0.1.0")
    )]
    fn empty() -> List<T> { Nil }

    /// Peek at the top element of the stack.
    ///
    /// Peek at the top element of the stack. Returns an `Option<&T>`
    /// with a borrowed pointer to the top element, or `None` if the
    /// stack is empty.
    ///
    /// # Examples:
    /// ```
    /// # use seax_svm::slist::{List,Stack};
    ///
    /// let mut s: List<isize> = Stack::empty();
    /// s = s.push(2);
    /// s = s.push(1);
    /// let pop_result = s.pop().unwrap();
    /// s = pop_result.1;
    /// assert_eq!(s.peek(), Some(&2));
    /// assert_eq!(pop_result.0, 1);
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="stack", since="0.1.0") )]
    fn peek(&self) -> Option<&T> {
        match self {
            &Nil => None,
            &Cons(ref it,_) => Some(it)
        }
    }

}

/// Singly-linked `cons` list.
///
/// This is used internally to represent list primitives in the
/// Seax virtual machine.
///
// TODO: potentially, a pointer to the last itemof the list could be
// cached using a `RefCell` or something to speed up access for
// appends/tail access. We could also check the length and decide whether
// to link hop from the head or tail when indexing. It would be necessary
// to investigate whether the added overhead of caching (both in terms of
// space and in terms of time taken to update the cache) would be worth
// the performance benefits --- my guess is that caching is worth the added
// costs (as usual).
#[derive(PartialEq,Clone)]
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
pub enum List<T> {
    /// Cons cell containing a `T` and a link to the tail
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    Cons(T, Box<List<T>>),
    /// The empty list.
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    Nil,
}

/// Public implementation for List.
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
impl<T> List<T> {


    /// Creates a new empty list
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    #[inline]
    pub fn new() -> List<T> { Nil }

    /// Prepends the given item to the list.
    ///
    /// This is an O(1) operation.
    ///
    /// # Arguments
    ///
    ///  + `item` - the item to prepend
    ///
    /// # Return Value
    ///
    ///  + The list with the new head item prepended
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let mut a_list: List<isize> = List::new();
    /// a_list = a_list.prepend(1);
    /// assert_eq!(a_list, list![1]);
    ///
    /// a_list = a_list.prepend(2);
    /// assert_eq!(a_list, list![2,1]);
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    pub fn prepend(self, it: T) -> List<T> { Cons(it, Box::new(self)) }

    /// Appends an item to the end of the list.
    ///
    /// This is an O(_n_) operation.
    ///
    /// # Arguments
    ///
    ///  + `item` - the item to append
    ///
    /// # Examples
    /// ```
    /// # #![feature(list)]
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let mut a_list: List<isize> = List::new();
    /// a_list.append(1);
    /// assert_eq!(a_list, list![1]);
    ///
    /// a_list.append(2);
    /// assert_eq!(a_list, list![1,2]);
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.3") )]
    pub fn append(&mut self, it: T) {
        match *self {
            Cons(_, ref mut tail) => (*tail).append(it),
            Nil => *self = Cons(it, Box::new(Nil))
        }

    }

    /// Appends an item to the end of the list.
    ///
    /// Returns the last element of the list to support 'append chaining'
    /// of a large number of items; this allows you to omit a complete traversal
    /// of the list for every append and should be used in situations
    /// such as `fold`s.
    ///
    /// The first append is still O(_n_), but long as you hang on to your
    /// pointer to the tail, subsequent appends should all be O(1). However,
    /// this requires you to keep a `&mut` pointer to the list, so use it
    /// sparingly, especially in situations of concurrent access.
    ///
    /// # Arguments
    ///
    ///  + `item` - the item to append
    ///
    /// # Examples
    /// ```
    /// # #![feature(list)]
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let mut a_list: List<isize> = List::new();
    ///
    ///     // this is a function so that the `&mut` borrow is released.
    ///     fn append_two_items<T>(l: &mut List<T>, first: T, second: T) {
    ///         l.append_chain(first).append_chain(second);
    ///     }
    ///
    /// append_two_items(&mut a_list, 1, 2);
    /// assert_eq!(a_list, list![1,2]);
    /// # }
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.3") )]
    pub fn append_chain(&mut self, it: T) -> &mut List<T> {
        match *self {
            Cons(_, ref mut tail) => (*tail).append_chain(it),
            Nil => { *self = Cons(it, Box::new(Nil)); self }
        }

    }

    /// Returns the length of the list.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let a_list = list!(1,2,3,4);
    /// assert_eq!(a_list.length(), 4)
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    pub fn length (&self) -> usize {
        match *self {
            Cons(_, ref tail) => 1 + tail.length(),
            Nil => 0
        }
    }

    /// Returns true if the list is empty, false otherwise.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let list = list!(1,2,3,4);
    /// assert_eq!(list.is_empty(), false)
    /// # }
    /// ```
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let list: List<isize> = Nil;
    /// assert_eq!(list.is_empty(), true)
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.8") )]
    pub fn is_empty (&self) -> bool {
        match *self {
            Cons(_,_) => false,
            Nil       => true
        }
    }

    /// Returns the tail of the list from this element.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let list = list!(1,2,3,4);
    /// assert_eq!(list.tail(), &list!(2,3,4))
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.8") )]
    pub fn tail<'a>(&'a self) -> &'a Self {
        match self {
            &Cons(_, ref cdr) => cdr,
            nil @ &Nil        => nil
        }
    }

    /// Provide a forward iterator
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator{current: self}
    }

    /// Returns the last element of the list
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let a_list = list!(1,2,3,4);
    /// assert_eq!(a_list.last(), &4)
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    pub fn last(&self) -> &T {
        match *self {
            Cons(ref car, ref cdr) if cdr.is_empty() => &car,
            Cons(_, ref cdr) => (*cdr).last(),
            Nil => panic!("Last called on empty list")
        }
    }


    /// Optionally index the list.
    ///
    /// Unlike list indexing syntax (`list[i]`), this returns `None`
    /// if the index is out of bound rather than panicking.
    ///
    /// Lists are zero-indexed, so just as when using list indexing syntax,
    /// the head of the list is index 0 and the last element of the list is
    /// index (length - 1).
    ///
    /// # Arguments
    ///
    ///  + `idx` - the index to attempt to access
    ///
    /// # Return Value
    ///
    ///   + `Some(&T)` if the index exists within the list, `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist::List::{Cons,Nil};
    /// # fn main() {
    /// let a_list = list!(1,2,3,4);
    /// assert_eq!(a_list.get(1), Some(&2));
    /// assert_eq!(a_list.get(3), Some(&4));
    /// assert_eq!(a_list.get(10), None);
    /// # }
    /// ```
    #[cfg_attr(feature = "nightly",
    stable(feature="list",since="0.3.0")
)]
    pub fn get<'a>(&'a self, index: u64) -> Option<&'a T> {
        match (0..index).fold(self, |acc, _| acc.tail()) {
            &Cons(ref car,_) => Some(car),
            &Nil             => None
        }

    }
}

#[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.5") )]
impl<'a, T> fmt::Display for List<T> where T: fmt::Display {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.5") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.iter();
        write!(f, "({}{})", it.next().unwrap(), it.fold(
            String::new(),
            |mut a, i| { a.push_str(format!(", {}", i).as_ref()); a} )
        )
    }
}

#[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.5") )]
impl<'a, T> fmt::Debug for List<T> where T: fmt::Debug {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.5") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cons(ref head, ref tail) => write!(f, "({:?} . {:?})", head, tail),
            Nil => write!(f,"nil")
        }
    }

}


#[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.3") )]
impl<T> FromIterator<T> for List<T> {
    /// Build a `List<T>` from a structure implementing `IntoIterator<T>`.
    ///
    /// This takes advantage of the `List.append_chain()` method under the
    /// hood to provide roughly O(_n_) performance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seax_svm::slist::List;
    /// # use std::iter::FromIterator;
    /// let mut a_vec = vec![1,2,3,4];
    /// let another_vec = a_vec.clone();
    /// let a_list = List::from_iter(a_vec);
    /// for i in 0..a_list.length() {
    ///     assert_eq!(a_list[i], another_vec[i])
    /// }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.3") )]
    fn from_iter<I>(iterable: I) -> List<T> where I: IntoIterator<Item=T> {
            let mut result  = List::new();
            iterable
                .into_iter()
                .fold(&mut result, |l, it| l.append_chain(it));
            result
    }

}

/// Wraps a List<T> to allow it to be used as an Iterator<T>
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
pub struct ListIterator<'a, T:'a> { current: &'a List<T> }

/// Implementation of Iterator for List. This allows iteration by
/// link hopping.
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    /// Get the next element from the list.
    ///
    /// Get the next element from the list. Returns a `Some<T>`, or `Nil`
    /// if at the end of the list.
    ///
    /// # Examples:
    /// ```
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons, Nil};
    /// # fn main () {
    /// let list = list!(1,2,3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next().unwrap(), &1);
    /// assert_eq!(iter.next().unwrap(), &2);
    /// assert_eq!(iter.next().unwrap(), &3);
    /// # }
    /// ```
    /// ```
    /// # #![feature(convert)]
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons, Nil};
    /// # fn main () {
    /// let l: List<isize> = list!(1,2,3,4,5,6);
    /// let mut string = String::new();
    /// for item in l.iter() {
    ///     string.push_str((item.to_string() + ", ").as_ref());
    /// }
    /// assert_eq!(string, "1, 2, 3, 4, 5, 6, ".to_string())
    /// # }
    /// ```
    #[inline]
    #[cfg_attr(feature = "nightly",
    stable(feature="list", since="0.2.8")
)]
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            &Cons(ref head, ref tail) => {
                self.current = &(**tail);
                Some(head)
                },
            &Nil => None
        }
    }
}

#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
impl<'a, T> ExactSizeIterator for ListIterator<'a, T> {
    fn len(&self) -> usize {
        self.current.length()
    }
}
/// Implementation of indexing for `List<T>`.
///
/// # Examples:
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list: List<isize> = list!(1,2,3,4,5,6);
/// assert_eq!(list[0usize], 1);
/// # }
/// ```
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
impl<T> Index<usize> for List<T> {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    type Output = T;

    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.8") )]
    fn index<'a>(&'a self, _index: usize) -> &'a T {
        &self[_index as u64]
    }
}

/// Implementation of indexing for `List<T>`.
///
/// # Examples:
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list = list!(1,2,3,4,5,6);
/// assert_eq!(list[0usize], 1);
/// # }
/// ```
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.0") )]
impl<T> Index<u64> for List<T> {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.0") )]
    type Output = T;

    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.2.8") )]
    fn index<'a>(&'a self, _index: u64) -> &'a T {
        self.get(_index)
            .expect(&format!("list index {} out of range", _index))
    }
}

/// Implementation of indexing for `List<T>`.
///
/// # Examples:
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list = list!(1,2,3,4,5,6);
/// assert_eq!(list[0isize], 1);
/// # }
/// ```
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
#[cfg_attr(feature = "nightly",
    deprecated(since="0.2.0", reason="use unsigned indices instead")
)]
impl<T> Index<i64> for List<T> {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    #[cfg_attr(feature = "nightly",
        deprecated(since="0.2.0", reason="use unsigned indices instead")
    )]
    type Output = T;

    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    #[cfg_attr(feature = "nightly",
        deprecated(since="0.2.0", reason="use unsigned indices instead")
    )]
    fn index<'a>(&'a self, _index: i64) -> &'a T {
        if _index < 0 {
            panic!("attempt to access negative index {}", _index)
        } else {
            &self[_index as u64]
        }
    }
}
/// Implementation of indexing for `List<T>`.
///
/// # Examples:
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list = list!(1,2,3,4,5,6);
/// assert_eq!(list[0isize], 1);
/// # }
/// ```
#[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
#[cfg_attr(feature = "nightly",
    deprecated(since="0.2.0", reason="use unsigned indices instead")
)]
impl<T> Index<isize> for List<T> {
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    #[cfg_attr(feature = "nightly",
        deprecated(since="0.2.0", reason="use unsigned indices instead")
    )]
    type Output = T;

    #[inline]
    #[cfg_attr(feature = "nightly", stable(feature="list", since="0.1.0") )]
    #[cfg_attr(feature = "nightly",
        deprecated(since="0.2.0", reason="use unsigned indices instead")
    )]
    fn index<'a>(&'a self, _index: isize) -> &'a T {
        if _index < 0 {
            panic!("attempt to access negative index {}", _index)
        } else {
            &self[_index as u64]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{List, Stack};
    use super::List::{Cons,Nil};

    #[test]
    fn test_list_length() {
        let full_list: List<i32> = list!(1i32, 2i32, 3i32);
        let empty_list: List<i32> = List::new();
        assert_eq!(full_list.length(), 3);
        assert_eq!(empty_list.length(), 0);
    }

    #[test]
    fn test_list_to_string() {
        let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        assert_eq!(l.to_string(), "(1, 2, 3)");
    }

    #[test]
    fn test_stack_length() {
        let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
        let empty_stack: List<i32> = Stack::empty();
        assert_eq!(full_stack.length(), 3);
        assert_eq!(empty_stack.length(), 0);
    }

    #[test]
    fn test_stack_peek() {
        let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
        let empty_stack: List<i32> = Stack::empty();
        assert_eq!(full_stack.peek(), Some(&1));
        assert_eq!(empty_stack.peek(), None);
    }

    #[test]
    fn test_stack_push() {
        let mut s: List<i32> = Stack::empty();
        assert_eq!(s.peek(), None);
        s = s.push(1);
        assert_eq!(s.peek(), Some(&1));
        s = s.push(6);
        assert_eq!(s.peek(), Some(&6));
    }

    #[test]
    fn test_stack_pop() {
        let mut s: List<i32> = Stack::empty();
        assert_eq!(s.peek(), None);
        s = s.push(1);
        assert_eq!(s.peek(), Some(&1));
        s = s.push(6);
        assert_eq!(s.peek(), Some(&6));
        let pop_result = s.pop().unwrap(); // should not break
        s = pop_result.1;
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(pop_result.0, 6);
    }

    #[test]
    fn test_list_usize_indexing() {
        let l: List<isize> = list!(1,2,3,4,5,6);
        assert_eq!(l[0usize],1);
        assert_eq!(l[1usize],2);
        assert_eq!(l[2usize],3);
        assert_eq!(l[3usize],4);
        assert_eq!(l[4usize],5);
        assert_eq!(l[5usize],6);
    }

    #[test]
    fn test_list_isize_indexing() {
        let l: List<isize> = list!(1,2,3,4,5,6);
        assert_eq!(l[0isize],1);
        assert_eq!(l[1isize],2);
        assert_eq!(l[2isize],3);
        assert_eq!(l[3isize],4);
        assert_eq!(l[4isize],5);
        assert_eq!(l[5isize],6);
    }

    #[test]
    fn test_list_macro() {
        let l: List<i32> = list!(1i32, 2i32, 3i32);
        assert_eq!(l.to_string(), "(1, 2, 3)")
    }

    #[test]
    fn test_list_iter() {
        let l: List<isize> = list!(1,2,3,4,5,6);
        let mut string = String::new();
        for item in l.iter() {
            string.push_str((item.to_string() + ", ").as_ref());
        }
        let slice: &str = string.as_ref(); // this is necessary because assert_eq! is weird
        assert_eq!(slice, "1, 2, 3, 4, 5, 6, ")
    }

}
