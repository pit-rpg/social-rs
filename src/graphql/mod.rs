extern crate tokio;

pub mod context;
pub mod routes;
pub mod schema;

use async_graphql::{ObjectType, Schema, SubscriptionType};
use std::result::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()

pub async fn dump_schema_to_disk<Q, M, S>(schema: &Schema<Q, M, S>) -> Result<(), std::io::Error>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    let sdl = schema.sdl();
    let bytes = sdl.as_bytes();
    let mut file = File::create("graphql/schema.gql").await?;

    file.write_all(bytes).await?;

    Ok(())
}
