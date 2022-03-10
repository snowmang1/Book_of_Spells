// tree implementation

pub struct Node<T> {
    key: Box<T>,
}

impl<T> Node<T> {
    pub fn new(k: Box<T>) -> Self{
        Self {
            key: k,
        }
    }
    pub fn read(self) -> Box<T>{
        self.key
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn num_test1() {
        let set = Node::new(Box::new(10));
        assert_eq!(*set.read(), 10);
    }

    #[test]
    fn str_test1() {
        let set = Node::new(Box::new("hello"));
        assert_eq!(*set.read(), "hello");
    }

    #[test]
    fn dbl_test1() {
        let set = Node::new(Box::new(0.123));
        assert_eq!(*set.read(), 0.123);
    }

}
