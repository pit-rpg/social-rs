use crate::db::{utils::{CollectionUtils, map_id_to_string}, DBUser, Gender};
use async_graphql::{InputObject, SimpleObject};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{bson::doc, options::FindOptions, Database, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::result::Result;
use super::chat::{Chat};

lazy_static! {
    static ref ARGON_2_CONF: argon2::Config<'static> = argon2::Config::default();
}

pub struct User;

impl User {
    pub fn test_password(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
    }

    pub fn hash_password(password: &str, salt: ObjectId) -> String {
        argon2::hash_encoded(password.as_bytes(), &salt.bytes(), &ARGON_2_CONF).unwrap()
    }

    pub async fn register(db: &Database, data: InputUserLogin) -> Result<DBUser, &'static str> {
        println!("Register!!! {:?}", data);

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

        println!("new user!!! {:?}", user);

        let res = DBUser::to_collection(&db)
            .insert_one(&user, None)
            .await;

        if let Err(err) = res {
            println!("{:?}", err);
            return Err("cent create user 1");
        }

        let id = res
            .unwrap()
            .inserted_id
            .as_object_id()
            .ok_or("cent create user 2")?;

        println!("inserted user!!! {:?}", id);

        let _ = Chat::create_user_private(db, &id).await;

        println!("chat created!!! {:?}", id);


        Self::log_in(db, data).await
    }

    pub async fn log_in(db: &Database, data: InputUserLogin) -> Result<DBUser, &'static str> {
        let user = DBUser::to_collection(&db)
            .find_one(doc! {"name_user": data.name_user}, None)
            .await
            .or(Err("cent find user"))?;

        let user = user.ok_or("error: cant find user")?;
        let pass_ok = Self::test_password(&user.password, &data.password);

        if !pass_ok {
            return Err("incorrect Password");
        }

        Ok(user)
    }

    pub async fn gt_by_id(db: &Database, id: ObjectId) -> Option<OutputUser> {
        DBUser::to_collection(&db)
            .find_one(doc! {"_id": id}, None)
            .await
            .ok()?
            .map(|u| u.into())
    }

    pub async fn find_user(
        db: &Database,
        data: InputFindUser,
    ) -> Result<Vec<OutputUser>, &'static str> {
        let options = FindOptions::builder().limit(data.limit.unwrap_or(10) as i64).build();

        println!("===> FIND_USER: {:?}", data);

        let cursor = DBUser::to_collection(&db)
            .find(doc! {"name_user": {"$regex": data.name_user}}, Some(options))
            .await
            .or(Err("cent find user"))?;

        let res = cursor
            .map(|item| match item {
                Ok(val) => Ok(val.into()),
                Err(_) => Err("cent get user"),
            })
            .try_collect::<Vec<OutputUser>>()
            .await
            .or(Err("cent get users"))?;

        Ok(res)
    }

    pub async fn get_user(
        db: &Database,
        id: String,
    ) -> Result<OutputUser, &'static str> {
        let id = ObjectId::parse_str(&id)
            .or(Err("cent parse user id"))?;


        DBUser::to_collection(&db)
            .find_one(doc!{"_id": id}, None)
            .await
            .or(Err("cent get user"))?
            .ok_or("cent get user")
            .map(|user| user.into())
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

#[derive(SimpleObject, Debug, Serialize, Deserialize, Default, Clone)]
pub struct OutputUser {
    id: Option<String>,
    name_user: Option<String>,
    name_display: Option<String>,
    gender: Option<Gender>,
    mail: Option<String>,
    age: Option<u32>,
}

impl From<&DBUser> for OutputUser {
    fn from(item: &DBUser) -> OutputUser {
        OutputUser {
            id: map_id_to_string(&item.id),
            name_user: Some(item.name_user.clone()),
            age: item.age,
            gender: Some(item.gender),
            mail: item.mail.clone(),
            name_display: item.name_display.clone(),
        }
    }
}

impl From<DBUser> for OutputUser {
    fn from(item: DBUser) -> OutputUser {
        (&item).into()
    }
}

