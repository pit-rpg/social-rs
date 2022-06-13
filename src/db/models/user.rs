use crate::db::{validations::validate_trimmed, utils::CollectionUtils};
use async_graphql::{Enum, InputObject, SimpleObject};
use mongodb::{bson::doc, bson::Uuid, options::IndexOptions, Collection, Database, IndexModel, options::FindOptions};
use serde::{Deserialize, Serialize};
use std::result::Result;
use validator::Validate;
use futures::stream::{StreamExt, TryStreamExt};

lazy_static! {
    static ref ARGON_2_CONF: argon2::Config<'static> = argon2::Config::default();
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Gender {
    None,
    Male,
    Female,
    Other,
}

impl std::default::Default for Gender {
    fn default() -> Self {
        Gender::None
    }
}

pub struct User;

impl User {
    pub async fn create_indexes(db: &Database) {
        let options = IndexOptions::builder().unique(true).build();

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

    pub fn test_password(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
    }

    pub fn hash_password(password: &str, salt: Uuid) -> String {
        argon2::hash_encoded(password.as_bytes(), &salt.bytes(), &ARGON_2_CONF).unwrap()
    }

    pub async fn register(db: &Database, data: InputUserLogin) -> Result<DBUser, &'static str> {
        let password_salt = Uuid::default();
        let password = Self::hash_password(&data.password, password_salt);

        let user = DBUser {
            name_user: data.name_user.trim().to_string(),
            password,
            // password_salt,
            ..DBUser::default()
        };

        DBUser::to_collection(&db)
            .insert_one(user, None)
            .await
            .or(Err("cent create user"))?;

        Self::log_in(db, data).await
    }

    pub async fn log_in(db: &Database, data: InputUserLogin) -> Result<DBUser, &'static str> {
        let user = DBUser::to_collection(&db)
            .find_one(doc! {"name_user": data.name_user}, None)
            .await
            .or(Err("cent find user"))?;

        let user = user.expect("error: cant find user");
        let pass_ok = Self::test_password(&user.password, &data.password);

        if !pass_ok {
            return Err("incorrect Password");
        }

        Ok(user)
    }

    pub async fn gt_by_id(db: &Database, id: Uuid) -> Result<DBUser, &'static str> {
        DBUser::to_collection(&db)
            .find_one(doc! {"_id": id}, None)
            .await
            .or(Err("cent find user"))?
            .ok_or("cent find user")
    }

    pub async fn find_user(db: &Database, data: InputFindUser) -> Result<Vec<OutputUser>, &'static str> {
        let options = FindOptions::builder()
            .limit(data.limit)
            .build();

        let cursor = DBUser::to_collection(&db)
            .find(doc! {"name_user": data.name_user}, Some(options))
            .await
            .or(Err("cent find user"))?;

        let res = cursor
            .map( |item| match item {
                Ok(val) => {Ok(val.into())},
                Err(_) => {Err("cent get user")},
            })
            .try_collect::<Vec<OutputUser>>()
            .await
            .or(Err("cent get users"))?;

        Ok(res)
    }
}

#[derive(InputObject)]
pub struct InputUserLogin {
    #[graphql(validator(min_length = 1, max_length = 128))]
    name_user: String,

    #[graphql(validator(min_length = 1, max_length = 256))]
    password: String,
}

#[derive(InputObject)]
pub struct InputFindUser {
    #[graphql(validator(min_length = 1, max_length = 128))]
    name_user: String,

    #[graphql(default = 10, validator(minimum = 1, maximum = 100))]
    limit: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default, Clone)]
pub struct DBUser {
    #[serde(rename = "_id")]
    pub id: Uuid,

    #[validate(length(min = 1, max = 128), custom = "validate_trimmed")]
    pub name_user: String,

    #[validate(length(min = 1, max = 128), custom = "validate_trimmed")]
    pub name_display: Option<String>,

    #[validate(length(min = 1, max = 256))]
    password: String,

    // password_salt: Uuid,
    pub gender: Gender,

    #[validate(email)]
    pub mail: Option<String>,

    #[validate(range(min = 18, max = 20))]
    pub age: Option<u32>,

    #[validate(phone)]
    pub phone: Option<String>,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize, Default, Clone)]
pub struct OutputUser {
    id: Option<String>,
    name_user: Option<String>,
    name_display: Option<String>,
    gender: Option<Gender>,
    mail: Option<String>,
    age: Option<u32>,
    phone: Option<String>,
}

impl From<&DBUser> for OutputUser {
    fn from(item: &DBUser) -> OutputUser {
        OutputUser {
            id: Some(item.id.to_string()),
            name_user: Some(item.name_user.clone()),
            age: item.age,
            gender: Some(item.gender),
            mail: item.mail.clone(),
            name_display: item.name_display.clone(),
            phone: item.phone.clone(),
        }
    }
}

impl From<DBUser> for OutputUser {
    fn from(item: DBUser) -> OutputUser {
        (&item).into()
    }
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
