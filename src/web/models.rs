#[deriving(Clone)]
pub struct Todo {
    pub id: int,
    pub description: ~str,
    pub completed: bool
}

impl Todo {
    pub fn new(name: &str) -> Todo {
        Todo {
            id: 3,
            description: name.to_owned(),
            completed: false
        }
    }
}
