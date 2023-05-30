use entity::async_graphql;

pub mod plan;
pub mod note;

pub use plan::MapQuery;
pub use note::NoteQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(NoteQuery, MapQuery);
