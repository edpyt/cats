use crate::{
    application::{repository::Repository, usecase::cat::CreateCatUseCase},
    domain::entities::cat::Cat,
    infrastructure::db::repositories::cat::CatRepository,
};

pub struct CreateCatUseCaseHandler {
    cat_repository: CatRepository,
}

impl CreateCatUseCaseHandler {
    pub fn new(cat_repository: CatRepository) -> Self {
        CreateCatUseCaseHandler { cat_repository }
    }

    pub fn execute(&self, request: CreateCatUseCase) {
        let id = self.cat_repository.next_identity();
        let cat = Cat::new(id, request.name, request.age, request.breed);
        self.cat_repository.create(cat.into()).unwrap();
    }
}
