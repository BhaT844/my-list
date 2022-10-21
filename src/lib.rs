mod node;

pub use crate::node::Node;

pub struct List<T> {
    node: Option<Node<T>>,
    size: usize,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { node: None, size: 0 }
    }

    pub fn to_vector(&self) -> Vec<&T> {
        let mut answer = Vec::new();

        if let Some(node) = &self.node {
            let mut iter = node;
            answer.push(&iter.value);
            while let Some(next_node) = &iter.next {
                answer.push(&next_node.value);
                iter = next_node;
            }
        }

        answer
    }

    // fn get(&self, index: usize) -> Option<T> {
        
    // }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value);

        if let Some(node) = self.node.take() {
            new_node.next = Some(Box::new(node));
        }

        self.node = Some(new_node);
        self.size += 1;
    }
}