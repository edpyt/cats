use sea_orm::{entity::prelude::*, ActiveValue};

use crate::application::dto::cat::CatDTO;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cat")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    // TODO: Need to change this to Uuid.
    pub id: String,
    pub name: String,
    pub age: i32,
    pub breed: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<CatDTO> for ActiveModel {
    fn from(cat_dto: CatDTO) -> Self {
        ActiveModel {
            id: ActiveValue::Set(cat_dto.id),
            name: ActiveValue::Set(cat_dto.name),
            age: ActiveValue::Set(cat_dto.age),
            breed: ActiveValue::Set(cat_dto.breed),
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use sea_orm::ActiveValue;
    use uuid::Uuid;

    use crate::application::dto::cat::CatDTO;

    use super::ActiveModel as CatModel;

    #[rstest]
    fn test_create_cat_model() {
        let cat = CatModel {
            id: ActiveValue::Set(String::from("test")),
            name: ActiveValue::Set(String::from("test")),
            age: ActiveValue::Set(8),
            breed: ActiveValue::Set(String::from("test")),
        };

        assert_eq!(cat.id, ActiveValue::Set(String::from("test")));

        let cat_id: String = Uuid::new_v4().to_string();
        let cat_dto = CatDTO {
            id: cat_id.clone(),
            name: String::from("test"),
            age: 12,
            breed: String::from("test"),
        };
        let cat: CatModel = cat_dto.into();

        assert_eq!(cat.id, ActiveValue::Set(cat_id));
    }
}
