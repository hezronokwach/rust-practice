
#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Add a worker at the start of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        // Create a new Worker with the provided role and name
        // Set its next pointer to the current head of the list
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(), // Take ownership of the current head
        };

        // Set the new worker as the head of the list
        self.grade = Some(Box::new(new_worker));
    }

    // Remove the last added worker (the head of the list)
    pub fn remove_worker(&mut self) -> Option<String> {
        // Take ownership of the current head
        self.grade.take().map(|worker| {
            // Update the head to point to the next worker
            self.grade = worker.next;
            worker.name
        })
    }

    // Return the name and role of the last added worker
    pub fn last_worker(&self) -> Option<(String, String)> {
        // If there's a worker at the head of the list
        self.grade.as_ref().map(|worker| {
            (worker.name.clone(), worker.role.clone())
        })
    }
}
