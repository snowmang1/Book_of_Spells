#![cfg(test)]

use super::*;

#[test]
fn init_node_test() {
    let n = Node {next: None, key: 0};
    let n_test: Node = Default::default();
    assert_eq!(n, n_test)
}
#[test]
fn init_queue_test() {
    let q = Queue {head: Some(Box::<Node>::new(Default::default())), size: 1};
    let n: Node = Default::default();
    assert_eq!(*q.head.unwrap(), n)
}
#[test]
fn queue_default_test() {
    let q: Queue = Default::default();
    let q_test = Queue {head: None, size: 0};
    assert_eq!(q, q_test)
}

#[test]
fn queue_new_test() {
    let q = Queue::new();
    let q_test = Queue {head: None, size: 0};
    assert_eq!(q.size, 0);
    assert_eq!(q, q_test)
}
#[test]
fn push_simple_test() {
    let mut q = Queue::new();
    let n1 = Node {next: None, key: 1};
    q.push(n1);
    let n_test = Node {next: None, key: 1};
    assert_eq!(q.size, 1);
    assert_eq!(n_test, *q.head.unwrap())
}
#[test]
fn push_two_test() {
    let mut q = Queue::new();
    let n1 = Node {next: None, key: 1};
    q.push(n1);
    let n2 = Node {next: None, key: 2};
    q.push(n2);
    let n_test = Node {next: Some(Box::new(Node{next: None, key: 1})), key: 2};
    assert_eq!(q.size, 2);
    assert_eq!(n_test, *q.head.unwrap())
}

#[test]
fn pop_simple_test() {
    let mut q = Queue::new();
    let n1 = Node {next: None, key: 1};
    q.push(n1);
    let n2 = Node {next: None, key: 2};
    q.push(n2);
    let q_test = q.pop();
    let n_test = Node {next: Some(Box::new(Node{next: None, key: 1})), key: 2};
    assert_eq!(n_test, q_test.unwrap());
    assert_eq!(q.size, 1);
}
#[test]
fn pop_two_test() {
    let mut q = Queue::new();
    let n1 = Node {next: None, key: 1};
    q.push(n1);
    let n2 = Node {next: None, key: 2};
    q.push(n2);
    let n2 = Node {next: None, key: 3};
    q.push(n2);
    let _null = q.pop();
    let q_test = q.pop();
    let n_test = Node {next: Some(Box::new(Node{next: None, key: 1})), key: 2};
    assert_eq!(n_test, q_test.unwrap());
    assert_eq!(q.size, 1);
}
