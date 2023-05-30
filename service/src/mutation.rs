use ::entity::async_graphql::{InputType, Upload};
use ::entity::bookable_resource;
use ::entity::bookable_resource::Entity as Resource;
use ::entity::plan;
use ::entity::plan::Coordinate;
use ::entity::{note, note::Entity as Note};
use ::entity::plan::PlanNode;
use sea_orm::*;
use std::io::Read;

pub struct Mutation;

impl Mutation {
    pub async fn create_note(db: &DbConn, form_data: note::Model) -> Result<note::Model, DbErr> {
        let active_model = note::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        };
        let res = Note::insert(active_model).exec(db).await?;

        Ok(note::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn update_note_by_id(
        db: &DbConn,
        id: i32,
        form_data: note::Model,
    ) -> Result<note::Model, DbErr> {
        let note: note::ActiveModel = Note::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        note::ActiveModel {
            id: note.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_note(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let note: note::ActiveModel = Note::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        note.delete(db).await
    }

    pub async fn delete_all_notes(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Note::delete_many().exec(db).await
    }

    pub async fn create_plan(
        db: &DbConn,
        coordinate: Coordinate,
        file: Vec<u8>,
    ) -> Result<PlanNode, DbErr> {
        let active_model = plan::ActiveModel {
            coordinates: Set(coordinate.to_owned()),
            image: Set(file),
            ..Default::default()
        };
        let res = plan::Entity::insert(active_model).exec(db).await?;

        Ok(PlanNode {
            id: res.last_insert_id,
            resources: vec![],
        })
    }

    pub async fn create_bookable_resource(
        db: &DbConn,
        coordinate: bookable_resource::Coordinate,
        plan_id: i32,
    ) -> Result<bookable_resource::Model, DbErr> {
        let active_model = bookable_resource::ActiveModel {
            coordinate: Set(coordinate.to_owned()),
            plan_id: Set(plan_id),
            kind: Set(bookable_resource::BookableResourceKind::Workspace),
            qr_code: Set(None),
            ..Default::default()
        };

        let res = bookable_resource::Entity::insert(active_model)
            .exec(db)
            .await?;

        Ok(bookable_resource::Model {
            id: res.last_insert_id,
            coordinate,
            plan_id,
            kind: bookable_resource::BookableResourceKind::Workspace,
            qr_code: None,
        })
    }

    pub async fn remove_bookable_resource(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let resource: bookable_resource::ActiveModel = Resource::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find bookable resource.".to_owned()))
            .map(Into::into)?;

        resource.delete(db).await
    }
}
