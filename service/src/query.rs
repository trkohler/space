use ::entity::bookable_resource::{Coordinate, Model as ResourceModel, ResourceNode};
use ::entity::plan::{Model as PlanModel, PlanNode};
use ::entity::user::{Entity as UserEntity, Model as UserModel, UserNode};
use ::entity::{plan, plan::Entity as Plan};
use sea_orm::{DbConn, DbErr};

pub struct Query;

impl Query {
    pub async fn query_plan_and_resources(
        db: &DbConn,
        plan_id: i32,
    ) -> Result<Option<PlanNode>, DbErr> {
        let result: Vec<(PlanModel, Vec<ResourceModel>)> =
            Plan::find_with_resources(plan_id).all(db).await?;

        let (plan, resources) = result.first().ok_or(DbErr::RecordNotFound(
            "there is no plan with that id".to_string(),
        ))?;

        let resources = resources
            .iter()
            .map(|r| ResourceNode {
                id: r.id,
                qr_code: None,
                kind: r.kind.clone(),
                coordinate: Coordinate {
                    x: r.coordinate.x,
                    y: r.coordinate.y,
                },
            })
            .collect();

        Ok(Some(PlanNode {
            id: plan.id,
            resources,
        }))
    }

    pub async fn find_map_by_id(db: &DbConn, id: i32) -> Result<Option<plan::Model>, DbErr> {
        Plan::find_by_id(id).one(db).await
    }

    pub async fn get_plan_with_resources(db: &DbConn, plan_id: i32) -> Result<plan::Model, DbErr> {
        // need to review structure of queries first
        unimplemented!();
    }

    pub async fn get_user_by_email(db: &DbConn, email: String) -> Result<Option<UserNode>, DbErr> {
        let user: Option<UserModel> = UserEntity::find_by_email(email).one(db).await?;

        if let Some(user) = user {
            Ok(Some(UserNode {
                id: user.id,
                email: user.email,
                display_name: user.display_name,
                role: user.role,
            }))
        } else {
            Ok(None)
        }
    }
}
