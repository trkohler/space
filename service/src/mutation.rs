use ::entity::bookable_resource;
use ::entity::bookable_resource::Entity as Resource;
use ::entity::bookable_resource::ResourceNode;
use ::entity::plan;
use ::entity::plan::PlanNode;
use ::entity::user;
use ::entity::user::UserNode;
use chrono::*;
use ::entity::user::Role;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_plan(db: &DbConn, file: Vec<u8>) -> Result<PlanNode, DbErr> {
        let active_model = plan::ActiveModel {
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
    ) -> Result<ResourceNode, DbErr> {
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

        Ok(ResourceNode {
            id: res.last_insert_id,
            coordinate,
            kind: bookable_resource::BookableResourceKind::Workspace,
            qr_code: None,
        })
    }

    pub async fn remove_bookable_resource(
        db: &DbConn,
        plan_id: i32,
        coordinate: bookable_resource::Coordinate,
    ) -> Result<DeleteResult, DbErr> {
        let resource: bookable_resource::ActiveModel =
            Resource::find_by_coordinates(plan_id, coordinate)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find bookable resource.".to_owned()))
                .map(Into::into)?;

        resource.delete(db).await
    }

    pub async fn register_user(
        db: &DbConn,
        email: String,
        display_name: String,
    ) -> Result<UserNode, DbErr> {
        let existing_user = user::Entity::find_by_email(email.clone()).one(db).await?;
        println!("existing user: {:?}", existing_user);

        if let Some(existing_user) = existing_user {
            return Ok(UserNode {
                id: existing_user.id,
                email,
                display_name,
                role: existing_user.role,
            });
        }

        let active_model = user::ActiveModel {
            email: Set(email.clone()),
            display_name: Set(display_name.clone()),
            created_at: Set(Utc::now().naive_utc()),
            oauth_provider: Set(user::OauthProvider::Google),
            role: Set(Role::SpaceAdmin),
            ..Default::default()
        };

        let res = active_model.insert(db).await?;

        Ok(UserNode {
            id: res.id,
            email,
            display_name,
            role: Role::SpaceAdmin,
        })
    }
}
