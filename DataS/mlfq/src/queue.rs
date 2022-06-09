// this is my queue
mod tests;

#[derive(Default, Clone, Debug, PartialEq)]
struct Node {
    next: Option<Box<Node>>,
    key: u16,
}

#[allow(dead_code)]
impl Node {
    fn new(key: u16) -> Self{
        let n = Node {next: None, key};
        n
    }
}

#[allow(dead_code)]
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
    fn push(mut self, n: Node) -> Self {
        // main does not own n after this function call
        let n = Node {next: self.head, key: n.key}; // n points to previous head
        self.head = Some(Box::new(n)); // current head is now n
        self
    }
    fn pop(mut self) -> Self {
        // temperarily returns and unwraps head
        self.head = self.head.unwrap().next;
        self
    }
}
