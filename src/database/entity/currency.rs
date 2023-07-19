use std::f32::consts::E;

use sea_orm::{
    entity::prelude::*,
    ActiveValue::{NotSet, Set},
    IntoActiveModel,
};
use serde::{Deserialize, Serialize};

use crate::database::DbResult;

use super::{Currency, User};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "currency")]
pub struct Model {
    #[serde(skip)]
    #[sea_orm(primary_key)]
    pub id: u32,
    #[serde(skip)]
    pub user_id: u32,
    pub name: String,
    pub balance: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn create_default(user: &User, db: &DatabaseConnection) -> DbResult<()> {
        // Create models for initial currency values
        let items = ["MTXCurrency", "GrindCurrency", "MissionCurrency"]
            .into_iter()
            .map(|name| ActiveModel {
                id: NotSet,
                user_id: Set(user.id),
                name: Set(name.to_string()),
                balance: Set(0),
            });
        Entity::insert_many(items)
            .exec_without_returning(db)
            .await?;
        Ok(())
    }

    pub async fn get_from_user(user: &User, db: &DatabaseConnection) -> DbResult<Vec<Currency>> {
        user.find_related(Entity).all(db).await
    }

    pub async fn create_or_update(
        db: &DatabaseConnection,
        user: &User,
        name: String,
        value: u32,
    ) -> DbResult<Self> {
        if let Some(model) = user
            .find_related(Entity)
            .filter(Column::Name.eq(&name))
            .one(db)
            .await?
        {
            let value = model.balance.saturating_add(value);
            let mut model = model.into_active_model();
            model.balance = Set(value);
            model.update(db).await
        } else {
            ActiveModel {
                id: NotSet,
                user_id: Set(user.id),
                name: Set(name),
                balance: Set(value),
            }
            .insert(db)
            .await
        }
    }
}
