use entity::async_graphql;

pub mod bookable_resource;
pub mod login;
pub mod plan;
pub mod registration;

pub use bookable_resource::ResourceMutation;
pub use login::LoginMutation;
pub use plan::PlanMutation;
pub use registration::RegistrationMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    PlanMutation,
    ResourceMutation,
    RegistrationMutation,
    LoginMutation,
);
