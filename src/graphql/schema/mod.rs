pub mod query;
pub mod subscription;

use async_graphql::Schema;
use async_graphql::EmptyMutation;
use query::RootQuery;
use subscription::RootSubscription;

pub type AppSchema = Schema<RootQuery, EmptyMutation, RootSubscription>;

pub fn get_schema() -> AppSchema {
    Schema::build(RootQuery, EmptyMutation, RootSubscription).finish()
}
