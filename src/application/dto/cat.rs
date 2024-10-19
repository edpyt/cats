use crate::domain::entities::cat::Cat;

#[derive(Debug)]
pub struct CatDTO {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub breed: String,
}

impl From<Cat> for CatDTO {
    fn from(cat: Cat) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            age: cat.age,
            breed: cat.breed,
        }
    }
}
