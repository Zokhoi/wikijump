//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "site_alias")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub alias_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: i64,
    pub site_id: i64,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub slug: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::SiteId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Site,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
