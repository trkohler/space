use entity::plan;
use entity::sea_orm::DbBackend;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::Schema;
use sea_orm_migration::prelude::*;
use crate::statements::{get_seaorm_create_stmt, get_seaorm_drop_stmt};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![get_seaorm_create_stmt(plan::Entity)];

        for stmt in stmts {
            manager.create_table(stmt.to_owned()).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![get_seaorm_drop_stmt(plan::Entity)];

        for stmt in stmts {
            manager.drop_table(stmt.to_owned()).await?;
        }

        Ok(())
    }
}

