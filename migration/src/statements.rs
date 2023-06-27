use crate::sea_orm::{DbBackend, Schema};
use entity::sea_orm::sea_query::{Table, TableCreateStatement, TableDropStatement};
use entity::sea_orm::EntityTrait;

pub fn get_seaorm_create_stmt<E: EntityTrait>(e: E) -> TableCreateStatement {
    let schema = Schema::new(DbBackend::Postgres);

    schema
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}

pub fn get_seaorm_drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}
