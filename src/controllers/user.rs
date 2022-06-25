use super::chat::ControllerChat;
use crate::db::{
    DBUser, Gender, {map_id_to_string, CollectionUtils},
};
use crate::error::{Error, GQLResult};
use async_graphql::{InputObject, Object};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, options::FindOptions, Database};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref ARGON_2_CONF: argon2::Config<'static> = argon2::Config::default();
}

pub struct ControllerUser;

impl ControllerUser {
    pub fn test_password(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
    }

    pub fn hash_password(password: &str, salt: ObjectId) -> String {
        argon2::hash_encoded(password.as_bytes(), &salt.bytes(), &ARGON_2_CONF).unwrap()
    }

    pub async fn register(db: &Database, data: InputUserLogin) -> GQLResult<DBUser> {
        let password_salt = ObjectId::default();
        let password = Self::hash_password(&data.password, password_salt);

        let user = DBUser {
            name_user: data.name_user.trim().to_string(),
            password,
            gender: Gender::None,
            age: None,
            id: None,
            mail: None,
            name_display: None,
            phone: None,
        };

        let res = DBUser::to_collection(&db).insert_one(&user, None).await?;

        let id = res.inserted_id.as_object_id().ok_or("cent create user")?;

        ControllerChat::create_user_private(db, &id).await?;

        Self::log_in(db, data).await
    }

    pub async fn log_in(db: &Database, data: InputUserLogin) -> GQLResult<DBUser> {
        let user = DBUser::to_collection(&db)
            .find_one(doc! {"name_user": data.name_user}, None)
            .await?;

        let user = user.ok_or("error: cant find user")?;
        let pass_ok = Self::test_password(&user.password, &data.password);

        if !pass_ok {
            Err("incorrect Password")?;
        }

        Ok(user)
    }

    pub async fn gt_by_id(db: &Database, id: ObjectId) -> Option<DBUser> {
        DBUser::to_collection(&db)
            .find_one(doc! {"_id": id}, None)
            .await
            .ok()?
    }

    pub async fn find_user(db: &Database, data: InputFindUser) -> GQLResult<Vec<DBUser>> {
        let options = FindOptions::builder()
            .limit(data.limit.unwrap_or(10) as i64)
            .build();

        let res = DBUser::to_collection(&db)
            .find(
                doc! {"name_user": {"$regex": data.name_user}},
                Some(options),
            )
            .await?
            .try_collect()
            .await?;

        Ok(res)
    }

    pub async fn get_user(db: &Database, id: String) -> GQLResult<DBUser> {
        let id = ObjectId::parse_str(&id).or(Err("cent parse user id"))?;

        DBUser::to_collection(&db)
            .find_one(doc! {"_id": id}, None)
            .await?
            .ok_or(Error::new_str("cent get user"))
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize, Clone)]
pub struct InputUserLogin {
    #[graphql(validator(min_length = 1, max_length = 128))]
    name_user: String,

    #[graphql(validator(min_length = 1, max_length = 256))]
    password: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize, Clone)]
pub struct InputFindUser {
    #[graphql(validator(min_length = 1, max_length = 128))]
    name_user: String,

    #[graphql(validator(minimum = 1, maximum = 100))]
    limit: Option<i32>,
}

#[Object(name="User")]
impl DBUser {
    async fn id(&self) -> Option<String> {
        map_id_to_string(&self.id)
    }
    async fn name_user(&self) -> Option<String> {
        Some(self.name_user.clone())
    }
    async fn name_display(&self) -> Option<String> {
        self.name_display.clone()
    }
    async fn gender(&self) -> Option<Gender> {
        Some(self.gender)
    }
    async fn mail(&self) -> Option<String> {
        self.mail.clone()
    }
    async fn age(&self) -> Option<u32> {
        self.age
    }
}
