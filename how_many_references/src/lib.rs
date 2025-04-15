pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list }
    }

    // Adds an element to the list in the Node
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // Removes all elements from the vector that are equal to the given value
    // This only happens if the two Rcs point to the same allocation
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        // Use retain to keep only elements that don't point to the same allocation
        self.ref_list.retain(|e| !Rc::ptr_eq(e, &element));
    }
}

// Returns how many times the value is referenced in the code
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    // Rc::strong_count returns the number of strong references to this allocation
    // We subtract 1 because the parameter itself counts as a reference
    Rc::strong_count(ref_list) - 1
}