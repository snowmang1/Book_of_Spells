#![cfg(test)]

use super::*;

#[test]
fn init_node_test() {
    let n = Node {next: None, key: 0};
    let n_test: Node = Default::default();
    assert_eq!(n, n_test);
}
#[test]
fn init_queue_test() {
    let q = Queue {head: Some(Box::<Node>::new(Default::default())), size: 1};
    let n: Node = Default::default();
    assert_eq!(*q.head.unwrap(), n);
}
#[test]
fn queue_default_test() {
    let q: Queue = Default::default();
    let q_test = Queue {head: None, size: 0};
    assert_eq!(q, q_test);
}

#[test]
fn queue_new_test() {
    let q = Queue::new();
    let q_test = Queue {head: None, size: 0};
    assert_eq!(q, q_test);
}
#[test]
fn pop_fail_test() {
    let q = Queue::new();
    assert_eq!(q.pop(), None);
}
#[test]
fn pop_simple_test() {
    let q = Queue::new();
    let n: Node = Default::default();
    assert_eq!(q.pop().unwrap(), n);
}
