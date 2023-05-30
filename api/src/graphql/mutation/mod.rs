use entity::async_graphql;

pub mod note;
pub mod plan;

pub use note::NoteMutation;
use crate::graphql::mutation::plan::PlanMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(NoteMutation, PlanMutation);
