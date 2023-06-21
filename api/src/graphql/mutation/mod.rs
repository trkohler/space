use entity::async_graphql;

pub mod note;
pub mod plan;
pub mod bookable_resource;

pub use note::NoteMutation;
pub use bookable_resource::ResourceMutation;
pub use plan::PlanMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(NoteMutation, PlanMutation, ResourceMutation);
