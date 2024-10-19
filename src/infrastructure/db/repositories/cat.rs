use sea_orm::{ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;

use crate::application::{dto::cat::CatDTO, repository::Repository};
use crate::infrastructure::db::models::cat;

pub struct CatRepository {
    db: DatabaseConnection,
}
impl CatRepository {
    pub async fn new(db: DatabaseConnection) -> Self {
        CatRepository { db }
    }
}

impl Repository for CatRepository {
    type DTO = CatDTO;

    fn next_identity(&self) -> String {
        Uuid::new_v4().to_string()
    }

    async fn async_create(&self, data: Self::DTO) -> Result<bool, Box<dyn std::error::Error>> {
        cat::ActiveModel::from(data).insert(&self.db).await?;
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, collections::HashMap, str::FromStr};

    use rstest::*;
    use sea_orm::sea_query::TableCreateStatement;
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema};
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use uuid::{Uuid, Version};

    use crate::application::{dto::cat::CatDTO, repository::Repository};
    use crate::infrastructure::db::models::cat;

    use super::CatRepository;

    struct CatRepositoryMock {
        cats: RefCell<HashMap<String, CatDTO>>,
    }

    impl CatRepositoryMock {
        fn new() -> Self {
            let cats = HashMap::new();
            CatRepositoryMock {
                cats: RefCell::new(cats),
            }
        }
    }

    impl Repository for CatRepositoryMock {
        type DTO = CatDTO;

        fn next_identity(&self) -> String {
            "test".to_string()
        }

        fn create(&self, data: Self::DTO) -> Result<bool, Box<dyn std::error::Error>> {
            self.cats.borrow_mut().insert(data.id.clone(), data);
            Ok(true)
        }
    }

    #[fixture]
    fn cat_repo_mock() -> CatRepositoryMock {
        CatRepositoryMock::new()
    }

    #[fixture]
    fn cat_dto() -> CatDTO {
        CatDTO {
            id: Uuid::new_v4().to_string(),
            name: "test".to_string(),
            age: 3,
            breed: "test".to_string(),
        }
    }

    #[rstest]
    fn create_cat_mock(cat_repo_mock: CatRepositoryMock, cat_dto: CatDTO) {
        let result = cat_repo_mock.create(cat_dto);

        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(cat_repo_mock.cats.borrow().len(), 1);
    }

    #[rstest]
    fn get_next_identity(cat_repo_mock: CatRepositoryMock) {
        let id = cat_repo_mock.next_identity();

        assert!(!id.is_empty());

        let uuid = Uuid::from_str(id.as_str());

        assert!(uuid.is_ok());
        assert_eq!(Some(Version::Random), uuid.unwrap().get_version())
    }

    // FIXME: #[rstest]
    #[tokio::test]
    async fn create_cat_sqlite() {
        let cat_dto: CatDTO = cat_dto();
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement = schema.create_table_from_entity(cat::Entity);
        let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();

        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        let cat_repo = CatRepository::new(db).await;
        let result = cat_repo.async_create(cat_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_cat_postgre() {
        let cat_dto: CatDTO = cat_dto();
        let node = Postgres::default().start().await.unwrap();
        let schema = Schema::new(DbBackend::Postgres);
        let stmt: TableCreateStatement = schema.create_table_from_entity(cat::Entity);
        let db: DatabaseConnection = Database::connect(&format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432).await.unwrap(),
        ))
        .await
        .unwrap();

        db.execute(db.get_database_backend().build(&stmt))
            .await
            .unwrap();
        let cat_repo = CatRepository::new(db).await;
        let result = cat_repo.async_create(cat_dto).await;

        assert!(result.is_ok());
    }
}
