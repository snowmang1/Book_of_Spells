// this is my queue
// queue must be declared mutable in order to use push or pop
mod tests;

#[derive(Default, Clone, Debug, PartialEq)]
struct Node {
    next: Option<Box<Node>>,
    key: u16,
}

impl Node {
    #![allow(dead_code)]
    fn new(key: u16) -> Self{
        Node {next: None, key}
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
struct Queue {
    head: Option<Box<Node>>, //current node
    size: u8,
}

impl Queue {
    #![allow(dead_code)]
    fn new() -> Self { //makes more sense than default
        let q: Queue = Default::default();
        q
    }
    fn push(&mut self, n: Node) -> () {
        // main does not own n after this function call
        let n = Node {next: self.head.to_owned(), key: n.key}; // n points to previous head
        self.head = Some(Box::new(n)); // current head is now n
        self.size += 1;
    }
    fn pop(&mut self) -> Result<Node, String> {
        let popped = match self.head.to_owned() {
            Some(x) => *x,
            None => panic!("ERROR: POPPED EMPTY QUEUE")
        };
        self.head = popped.next.to_owned();
        self.size -= 1;
        Ok(popped)
    }
}
