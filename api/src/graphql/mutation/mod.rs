use entity::async_graphql;

pub mod plan;
pub mod bookable_resource;
pub mod registration;
pub mod login;

pub use bookable_resource::ResourceMutation;
pub use plan::PlanMutation;
pub use registration::RegistrationMutation;
pub use login::LoginMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(PlanMutation, ResourceMutation, RegistrationMutation, LoginMutation);
