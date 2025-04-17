#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        // Create a new node with the given value
        let new_node = Node {
            value,
            next: None,
        };

        // If the list is empty, set the new node as the head
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        // Otherwise, create a new node that points to the current head
        let current_head = self.head.take().unwrap();
        let new_head = Node {
            value: new_node.value,
            next: Some(Box::new(current_head)),
        };

        // Update the head to the new node
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) {
        if self.head.is_none() {
            return;
        }

        let current_head = self.head.take().unwrap();
        
        // If the head has a next node, make it the new head
        if let Some(next_node) = current_head.next {
            self.head = Some(*next_node);
        }
        // Otherwise, the list is now empty
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_opt = &self.head;

        while let Some(node) = current_opt {
            count += 1;
            current_opt = &node.next.as_deref();
        }

        count
    }
}