#![cfg(test)]
use super::*;

#[test]
fn test_append_and_peek_tail() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);

    assert_eq!(list.peek_tail(), Some(&3));
}

#[test]
fn test_prepend_and_peek_head() {
    let mut list = LinkedList::new();
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);

    assert_eq!(list.peek_head(), Some(&3));
}

#[test]
fn test_pop_head() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    assert_eq!(list.pop_head(), Some(1));
    assert_eq!(list.pop_head(), Some(2));
    assert_eq!(list.pop_head(), None);
}

#[test]
fn test_pop_tail() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    assert_eq!(list.pop_tail(), Some(2));
    assert_eq!(list.pop_tail(), Some(1));
    assert_eq!(list.pop_tail(), None);
}

#[test]
fn test_empty_list_peek_and_pop() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.peek_head(), None);
    assert_eq!(list.peek_tail(), None);
    assert_eq!(list.pop_head(), None);
    assert_eq!(list.pop_tail(), None);
}

#[test]
fn test_single_element_behavior() {
    let mut list = LinkedList::new();
    list.append(10);
    assert_eq!(list.peek_head(), Some(&10));
    dbg!("working");
    assert_eq!(list.peek_tail(), Some(&10));
    dbg!("working");
    assert_eq!(list.pop_head(), Some(10));
    dbg!("working");
    assert!(list.tail.is_null());
    assert!(list.head.is_null());
    assert_eq!(list.pop_tail(), None);
    dbg!("all working");
}

#[test]
fn test_iterator() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);

    let collected: Vec<_> = list.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_mixed_operations() {
    let mut list = LinkedList::new();
    list.append(1);
    list.prepend(0);
    list.append(2);
    list.pop_head();
    list.pop_tail();
    list.prepend(-1);

    let collected: Vec<_> = list.into_iter().collect();
    assert_eq!(collected, vec![-1, 1]);
}
