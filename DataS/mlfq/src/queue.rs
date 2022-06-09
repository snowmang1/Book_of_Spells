// this is my queue
mod tests;

#[derive(Default, Clone, Debug, PartialEq)]
struct Node {
    next: Option<Box<Node>>,
    key: u16,
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
        self.head.unwrap().next = Some(Box::new(n.to_owned()));
        self.size += 1;
        self.head = Some(Box::new(n));
        self
    }
    fn pop(self) -> Option<Node> {
        Some(*self.head.unwrap())
    }
}
