#[deriving(Clone)]
pub struct Todo {
    id: int,
    description: ~str,
    completed: bool
}

impl Todo {
    pub fn new(name: ~str) -> Todo {
        Todo {
            id: 3,
            description: name,
            completed: false
        }
    }
}
