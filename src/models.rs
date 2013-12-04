#[deriving(Clone)]
pub struct Todo {
    description: ~str,
    completed: bool
}

impl Todo {
    pub fn new(name: ~str) -> Todo {
        Todo {
            description: name,
            completed: false
        }
    }
}
