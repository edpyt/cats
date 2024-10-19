pub struct CreateCatUseCase {
    pub name: String,
    pub age: i32,
    pub breed: String,
}

impl CreateCatUseCase {
    pub fn new(name: &str, age: i32, breed: &str) -> Self {
        Self {
            age,
            breed: breed.to_string(),
            name: name.to_string(),
        }
    }
}
