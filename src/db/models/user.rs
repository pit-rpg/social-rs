use crate::db::{utils::CollectionUtils, validations::validate_trimmed};
use async_graphql::{Enum};
use mongodb::{
    bson::doc, bson::oid::ObjectId, options::IndexOptions, Collection, Database,
    IndexModel,
};
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Gender {
    None,
    Male,
    Female,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DBUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1, max = 128), custom = "validate_trimmed")]
    pub name_user: String,

    #[validate(length(min = 1, max = 128), custom = "validate_trimmed")]
    pub name_display: Option<String>,

    #[validate(length(min = 1, max = 256))]
    pub password: String,

    pub gender: Gender,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub mail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 18, max = 20))]
    pub age: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(phone)]
    pub phone: Option<String>,
}

impl CollectionUtils<DBUser> for DBUser {
    fn to_collection(db: &Database) -> Collection<DBUser> {
        let name = Self::get_collection_name();
        db.collection::<DBUser>(name)
    }

    fn get_collection_name() -> &'static str {
        "User"
    }
}

impl DBUser {
    pub async fn create_indexes(db: &Database) {
        let options = IndexOptions::builder()
            .unique(true)
            .partial_filter_expression(doc!{
                "name_user": {"$type": "string"},
                "phone": {"$type": "string"},
                "mail": {"$type": "string"},
            })
            .build();

        let model1 = IndexModel::builder()
            .keys(doc! {"name_user": 1})
            .options(options.clone())
            .build();
        let model2 = IndexModel::builder()
            .keys(doc! {"phone": 1})
            .options(options.clone())
            .build();
        let model3 = IndexModel::builder()
            .keys(doc! {"mail": 1})
            .options(options.clone())
            .build();

        DBUser::to_collection(&db)
            .create_indexes(vec![model1, model2, model3], None)
            .await
            .expect("error creating index!");
    }
}
