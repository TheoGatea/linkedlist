mod tests;
use std::ptr::null_mut;

struct Node<T> {
    val: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> Node<T> {
    fn new(thing: T) -> Node<T> {
        Node {
            val: thing,
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: null_mut(),
            tail: null_mut(),
            size: 0,
        }
    }

    fn peek_head(&self) -> Option<&T> {
        if self.head.is_null() {
            assert!(self.size == 0);
            None
        } else {
            unsafe { Some(&(*self.head).val) }
        }
    }

    fn peek_tail(&self) -> Option<&T> {
        if self.tail.is_null() {
            assert!(self.size == 0);
            None
        } else {
            unsafe { Some(&(*self.tail).val) }
        }
    }

    fn prepend(&mut self, new_elem: T) {
        let new_elem = Box::into_raw(Box::new(Node::new(new_elem)));
        if self.head.is_null() {
            assert!(self.tail.is_null());
            assert!(self.size == 0);
            self.head = new_elem;
            self.tail = new_elem;
            self.size += 1;
        } else {
            assert!(self.size > 0);
            let old_head = self.head;
            unsafe {
                (*old_head).prev = new_elem;
            }
            unsafe {(*new_elem).next = old_head;}
            self.head = new_elem;
            self.size += 1;
        }
    }

    fn append(&mut self, new_elem: T) {
        let new_elem = Box::into_raw(Box::new(Node::new(new_elem)));
        if self.tail.is_null() {
            assert!(self.head.is_null());
            assert!(self.size == 0);
            self.head = new_elem;
            self.tail = new_elem;
            self.size += 1;
        } else {
            assert!(self.size > 0);
            let old_tail = self.tail;
            unsafe {
                (*old_tail).next = new_elem;
            }
            unsafe { (*new_elem).prev = old_tail };
            self.tail = new_elem;
            self.size += 1;
        }
    }

    fn pop_head(&mut self) -> Option<T> {
        if self.head.is_null() {
            assert!(self.size == 0);
            None
        } else {
            let old_head = self.head;
            let new_head = unsafe { (*old_head).next };
            if new_head.is_null() {
                self.head = null_mut();
                self.tail = null_mut();
                self.size -= 1;
                unsafe { Some(Box::from_raw(old_head).val) }
            } else {
                unsafe {
                    (*old_head).next = null_mut();
                }
                unsafe {
                    (*new_head).prev = null_mut();
                }
                self.head = new_head;
                self.size -= 1;
                unsafe { Some(Box::from_raw(old_head).val) }
            }
        }
    }

    fn pop_tail(&mut self) -> Option<T> {
        if self.tail.is_null() {
            assert!(self.size == 0);
            None
        } else {
            let old_tail = self.tail;
            let new_tail = unsafe { (*old_tail).prev };
            if new_tail.is_null() {
                self.tail = null_mut();
                self.head = null_mut();
                self.size -= 1;
                unsafe { Some(Box::from_raw(old_tail).val) }
            } else {
                unsafe {
                    (*old_tail).prev = null_mut();
                }
                unsafe {
                    (*new_tail).next = null_mut();
                }
                self.tail = new_tail;
                self.size -= 1;
                unsafe { Some(Box::from_raw(old_tail).val) }
            }
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_head() {}
    }
}
