pub mod input;
pub mod mutation;
pub mod query;
pub mod subscription;

use async_graphql::Schema;
use mutation::RootMutation;
use query::RootQuery;
use subscription::RootSubscription;

pub type AppSchema = Schema<RootQuery, RootMutation, RootSubscription>;

pub fn get_schema() -> AppSchema {
    Schema::build(RootQuery, RootMutation, RootSubscription).finish()
}
