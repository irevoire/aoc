use std::iter::{FromIterator, FusedIterator, IntoIterator};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct CyclicList<T> {
    nodes: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> CyclicList<T> {
    /// Create a new empty [CyclicList].
    ///
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let list: CyclicList<char> = CyclicList::new();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.is_empty(), true);
    /// assert_eq!(list.current(), None);
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: None,
            len: 0,
        }
    }

    /// Returns true if the [CyclicList] is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.is_empty(), true);
    /// list.push_right(42);
    /// assert_eq!(list.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_none()
    }

    /// Returns the length of the [CyclicList].
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let list: CyclicList<usize> = CyclicList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Provides a reference to the current element, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::current_mut].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.current(), None);
    /// list.push_right(42);
    /// assert_eq!(list.current(), Some(&42));
    /// ```
    pub fn current(&self) -> Option<&T> {
        unsafe { self.nodes.map(|node| node.as_ref().current()) }
    }

    /// Provides a mutable reference to the current element, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::current].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.current_mut(), None);
    /// list.push_right(42);
    /// *list.current_mut().unwrap() += 1;
    /// assert_eq!(list.current(), Some(&43));
    /// ```
    pub fn current_mut(&mut self) -> Option<&mut T> {
        unsafe { self.nodes.map(|mut node| node.as_mut().current_mut()) }
    }

    /// Provides a reference to the element on the right, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::right_mut], [CyclicList::left], [CyclicList::move_right], [CyclicList::push_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.right(), None);
    /// list.push_right(42);
    /// assert_eq!(list.right(), Some(&42));
    /// list.push_right(43);
    /// assert_eq!(list.right(), Some(&43));
    /// ```
    pub fn right(&self) -> Option<&T> {
        unsafe { self.nodes.map(|node| node.as_ref().right()) }
    }

    /// Provides a mutable reference to the element on the right, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::right], [CyclicList::left_mut], [CyclicList::move_right], [CyclicList::push_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.right_mut(), None);
    /// list.push_right(42);
    /// *list.right_mut().unwrap() += 1;
    /// assert_eq!(list.right(), Some(&43));
    /// ```
    pub fn right_mut(&mut self) -> Option<&mut T> {
        unsafe { self.nodes.map(|mut node| node.as_mut().right_mut()) }
    }

    /// Adds an element to the right of the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::push_left], [CyclicList::move_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_right(42); // this is the current element AND the element to the right
    /// assert_eq!(list.right(), Some(&42));
    /// list.push_right(43); // this is the element to the right  AND the left
    /// assert_eq!(list.right(), Some(&43));
    /// ```
    pub fn push_right(&mut self, el: T) {
        self.len += 1;
        if let Some(mut current) = self.nodes {
            unsafe {
                current.as_mut().push_right(el);
            }
        } else {
            self.nodes = Some(Node::new(el));
        }
    }

    /// Move to the right element of the [CyclicList].
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::push_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_right(42);
    /// list.push_right(43);
    /// assert_eq!(list.current(), Some(&42));
    /// list.move_right();
    /// assert_eq!(list.current(), Some(&43));
    /// list.move_right();
    /// ```
    pub fn move_right(&mut self) {
        unsafe {
            self.nodes = self.nodes.map(|node| node.as_ref().right);
        }
    }

    /// Adds an element to the right of the current element and define it as the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::push_left], [CyclicList::move_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_move_right(42);
    /// assert_eq!(list.current(), Some(&42));
    /// list.push_move_right(43);
    /// assert_eq!(list.current(), Some(&43));
    /// assert_eq!(list.left(), Some(&42));
    /// assert_eq!(list.right(), Some(&42));
    /// ```
    pub fn push_move_right(&mut self, el: T) {
        self.push_right(el);
        self.move_right();
    }

    /// Consume itself and return a new self that moved one element to the right.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::into_right_n], [CyclicList::move_right], [CyclicList::right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_right(42);
    /// list.push_right(43);
    /// assert_eq!(list.current(), Some(&42));
    /// let list = list.into_right();
    /// assert_eq!(list.current(), Some(&43));
    /// let list = list.into_right();
    /// assert_eq!(list.current(), Some(&42));
    /// ```
    pub fn into_right(mut self) -> Self {
        self.move_right();
        self
    }

    /// Adds an element to the right of the current element and define it as the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::push_left], [CyclicList::move_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let list = CyclicList::new();
    /// let list = list.push_into_right(42);
    /// assert_eq!(list.current(), Some(&42));
    /// let list = list.push_into_right(43);
    /// assert_eq!(list.current(), Some(&43));
    /// assert_eq!(list.left(), Some(&42));
    /// assert_eq!(list.right(), Some(&42));
    /// ```
    pub fn push_into_right(mut self, el: T) -> Self {
        self.push_right(el);
        self.into_right()
    }

    /// Consume itself and return a new self that moved `n` elements to the right.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// See also: [CyclicList::into_right], [CyclicList::move_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.current(), Some(&0));
    /// let list = list.into_right_n(2);
    /// assert_eq!(list.current(), Some(&2));
    /// ```
    pub fn into_right_n(self, n: usize) -> Self {
        (0..n).fold(self, |list, _| list.into_right())
    }

    /// Move to the right element of the [CyclicList] `n` times.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// See also: [CyclicList::move_right], [CyclicList::move_right_n].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.current(), Some(&0));
    /// list.move_right_n(2);
    /// assert_eq!(list.current(), Some(&2));
    /// ```
    pub fn move_right_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.move_right())
    }

    /// Pop the element to the right.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::push_right].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_left(42); // this is the current element
    /// list.push_right(43); // this is the element to the right AND to the left
    /// assert_eq!(list.pop_right(), Some(43));
    /// assert_eq!(list.pop_right(), Some(42));
    /// assert_eq!(list.pop_right(), None);
    /// ```
    pub fn pop_right(&mut self) -> Option<T> {
        if let Some(mut node) = self.nodes {
            self.len = self.len.saturating_sub(1);
            let ret = unsafe { Some(node.as_mut().pop_right()) };
            if self.len == 0 {
                self.nodes = None;
            }
            ret
        } else {
            None
        }
    }

    /// Pop the current element and move to the right.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::right], [CyclicList::pop_right], [CyclicList::move_right] .
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4, 5].iter().copied().collect();
    /// assert_eq!(list.pop_move_right(), Some(0));
    /// assert_eq!(list.pop_move_right(), Some(1));
    /// assert_eq!(list.pop_move_right(), Some(2));
    /// assert_eq!(list.pop_move_right(), Some(3));
    /// assert_eq!(list.pop_move_right(), Some(4));
    /// assert_eq!(list.pop_move_right(), Some(5));
    /// assert_eq!(list.pop_move_right(), None);
    /// ```
    pub fn pop_move_right(&mut self) -> Option<T> {
        self.move_right();
        self.pop_left()
    }

    /// Provides a reference to the element on the left, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::left_mut], [CyclicList::right], [CyclicList::move_left], [CyclicList::push_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.left(), None);
    /// list.push_left(42);
    /// assert_eq!(list.left(), Some(&42));
    /// list.push_left(43);
    /// assert_eq!(list.left(), Some(&43));
    /// ```
    pub fn left(&self) -> Option<&T> {
        unsafe { self.nodes.map(|node| node.as_ref().left()) }
    }

    /// Provides a mutable reference to the element on the left, or [None] if the list is empty.
    ///
    /// See also: [CyclicList::left], [CyclicList::right_mut], [CyclicList::move_left], [CyclicList::push_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// assert_eq!(list.left_mut(), None);
    /// list.push_left(42);
    /// *list.left_mut().unwrap() += 1;
    /// assert_eq!(list.left(), Some(&43));
    /// ```
    pub fn left_mut(&mut self) -> Option<&mut T> {
        unsafe { self.nodes.map(|mut node| node.as_mut().left_mut()) }
    }

    /// Adds an element to the left of the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::push_left], [CyclicList::move_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_left(42);
    /// assert_eq!(list.left(), Some(&42));
    /// list.push_left(43);
    /// assert_eq!(list.left(), Some(&43));
    /// ```
    pub fn push_left(&mut self, el: T) {
        self.len += 1;
        if let Some(mut current) = self.nodes {
            unsafe {
                current.as_mut().push_left(el);
            }
        } else {
            self.nodes = Some(Node::new(el));
        }
    }

    /// Move to the left element of the [CyclicList].
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::push_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_left(42);
    /// list.push_left(43);
    /// assert_eq!(list.current(), Some(&42));
    /// list.move_left();
    /// assert_eq!(list.current(), Some(&43));
    /// list.move_left();
    /// assert_eq!(list.current(), Some(&42));
    /// ```
    pub fn move_left(&mut self) {
        unsafe {
            self.nodes = self.nodes.map(|node| node.as_ref().left);
        }
    }

    /// Adds an element to the left of the current element and define it as the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::push_left], [CyclicList::move_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_move_left(42);
    /// assert_eq!(list.current(), Some(&42));
    /// list.push_move_left(43);
    /// assert_eq!(list.current(), Some(&43));
    /// assert_eq!(list.left(), Some(&42));
    /// assert_eq!(list.left(), Some(&42));
    /// ```
    pub fn push_move_left(&mut self, el: T) {
        self.push_left(el);
        self.move_left();
    }

    /// Move to the left element of the [CyclicList] `n` times.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// See also: [CyclicList::move_left], [CyclicList::move_right_n].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.current(), Some(&0));
    /// list.move_left_n(2);
    /// assert_eq!(list.current(), Some(&3));
    /// ```
    pub fn move_left_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.move_left())
    }

    /// Consume itself and return a new self that moved one element to the left.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::into_left_n], [CyclicList::move_left], [CyclicList::left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list = CyclicList::new();
    /// list.push_left(42);
    /// list.push_left(43);
    /// assert_eq!(list.current(), Some(&42));
    /// let list = list.into_left();
    /// assert_eq!(list.current(), Some(&43));
    /// let list = list.into_left();
    /// assert_eq!(list.current(), Some(&42));
    /// ```
    pub fn into_left(mut self) -> Self {
        self.move_left();
        self
    }

    /// Adds an element to the left of the current element and define it as the current element.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::push_left], [CyclicList::move_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let list = CyclicList::new();
    /// let list = list.push_into_left(42);
    /// assert_eq!(list.current(), Some(&42));
    /// let list = list.push_into_left(43);
    /// assert_eq!(list.current(), Some(&43));
    /// assert_eq!(list.left(), Some(&42));
    /// assert_eq!(list.left(), Some(&42));
    /// ```
    pub fn push_into_left(mut self, el: T) -> Self {
        self.push_left(el);
        self.into_left()
    }

    /// Consume itself and return a new self that moved `n` elements to the left.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// See also: [CyclicList::into_left], [CyclicList::move_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.current(), Some(&0));
    /// let list = list.into_left_n(2);
    /// assert_eq!(list.current(), Some(&3));
    /// ```
    pub fn into_left_n(self, n: usize) -> Self {
        (0..n).fold(self, |list, _| list.into_left())
    }

    /// Pop the element to the left.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::push_left].
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.pop_left(), Some(4));
    /// assert_eq!(list.pop_left(), Some(3));
    /// assert_eq!(list.pop_left(), Some(2));
    /// assert_eq!(list.pop_left(), Some(1));
    /// assert_eq!(list.pop_left(), Some(0));
    /// assert_eq!(list.pop_left(), None);
    /// ```
    pub fn pop_left(&mut self) -> Option<T> {
        if let Some(mut node) = self.nodes {
            self.len = self.len.saturating_sub(1);
            let ret = unsafe { Some(node.as_mut().pop_left()) };
            if self.len == 0 {
                self.nodes = None;
            }
            ret
        } else {
            None
        }
    }

    /// Pop the current element and move to the left.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// See also: [CyclicList::left], [CyclicList::pop_left], [CyclicList::move_left] .
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4].iter().copied().collect();
    /// assert_eq!(list.pop_move_left(), Some(0));
    /// assert_eq!(list.pop_move_left(), Some(4));
    /// assert_eq!(list.pop_move_left(), Some(3));
    /// assert_eq!(list.pop_move_left(), Some(2));
    /// assert_eq!(list.pop_move_left(), Some(1));
    /// assert_eq!(list.pop_move_left(), None);
    /// ```
    pub fn pop_move_left(&mut self) -> Option<T> {
        self.move_left();
        self.pop_right()
    }
}

impl<T> FromIterator<T> for CyclicList<T> {
    /// Create a new [CyclicList] from anything that implements [IntoIterator].
    ///
    /// # Examples
    /// ```
    /// use aoc::CyclicList;
    ///
    /// let mut list: CyclicList<usize> = [0, 1, 2, 3, 4, 5].iter().copied().collect();
    /// assert_eq!(list.len(), 6);
    /// assert_eq!(list.current(), Some(&0));
    /// assert_eq!(list.left(), Some(&5));
    /// assert_eq!(list.right(), Some(&1));
    ///
    /// list.move_right_n(3);
    /// assert_eq!(list.current(), Some(&3));
    /// assert_eq!(list.left(), Some(&2));
    /// assert_eq!(list.right(), Some(&4));
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let list = iter
            .into_iter()
            .fold(Self::new(), |list, element| list.push_into_right(element));
        list.into_right()
    }
}

pub struct Iter<'a, T: 'a> {
    list: CyclicList<T>,
    // The number of elements that have been returned. Once this reach the
    // size of the list, the iterator should returns [None].
    consumed: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.consumed == self.list.len() {
            None
        } else {
            self.consumed += 1;
            self.list.move_right();
            // we need to transmute the element since rust is not able to infer
            // that elements returned by the [CyclicList] have a lifetime of 'a
            unsafe { std::mem::transmute(self.list.current()) }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IntoIter<T> {
    list: CyclicList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_move_right()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<T> IntoIterator for CyclicList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> FusedIterator for IntoIter<T> {}
impl<T> ExactSizeIterator for IntoIter<T> {}

#[derive(Debug)]
struct Node<T> {
    left: NonNull<Node<T>>,
    right: NonNull<Node<T>>,
    element: T,
}

impl<T> Node<T> {
    pub fn new(element: T) -> NonNull<Self> {
        let ret = Self {
            left: NonNull::dangling(),
            right: NonNull::dangling(),
            element,
        };
        let mut ret = NonNull::new(Box::leak(Box::new(ret))).unwrap();
        unsafe {
            ret.as_mut().right = ret;
            ret.as_mut().left = ret;
            ret
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }

    pub fn current(&self) -> &T {
        &self.element
    }

    pub fn current_mut(&mut self) -> &mut T {
        &mut self.element
    }

    pub unsafe fn right(&self) -> &T {
        self.right.as_ref().current()
    }

    pub unsafe fn right_mut(&mut self) -> &mut T {
        self.right.as_mut().current_mut()
    }

    pub unsafe fn push_right(&mut self, element: T) {
        let mut node = Self::new(element);

        self.right.as_mut().left = node;
        node.as_mut().right = self.right;

        node.as_mut().left = NonNull::new(self).unwrap();
        self.right = node;
    }

    pub unsafe fn pop_right(&mut self) -> T {
        let ret = self.right.as_mut();

        ret.right.as_mut().left = ret.left;
        ret.left.as_mut().right = ret.right;

        let ret = Box::from_raw(ret as *mut Self);
        ret.into_element()
    }

    pub unsafe fn left(&self) -> &T {
        self.left.as_ref().current()
    }

    pub unsafe fn left_mut(&mut self) -> &mut T {
        self.left.as_mut().current_mut()
    }

    pub unsafe fn push_left(&mut self, element: T) {
        let mut node = Self::new(element);

        self.left.as_mut().right = node;
        node.as_mut().left = self.left;

        node.as_mut().right = NonNull::new(self).unwrap();
        self.left = node;
    }

    pub unsafe fn pop_left(&mut self) -> T {
        let ret = self.left.as_mut();

        ret.left.as_mut().right = ret.right;
        ret.right.as_mut().left = ret.left;

        let ret = Box::from_raw(ret as *mut Self);
        ret.into_element()
    }
}
