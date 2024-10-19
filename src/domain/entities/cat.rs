#[derive(Debug)]
pub struct Cat {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub breed: String,
}

impl Cat {
    pub fn new(id: String, name: String, age: i32, breed: String) -> Self {
        Cat {
            id,
            name,
            age,
            breed,
        }
    }
}
