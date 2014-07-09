#[deriving(Clone)]
pub struct Todo {
    pub id: int,
    pub description: String,
    pub completed: bool
}

impl Todo {
    pub fn new(name: &str) -> Todo {
        Todo {
            id: 3,
            description: String::from_str(name),
            completed: false
        }
    }
}
