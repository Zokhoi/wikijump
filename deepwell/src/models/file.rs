//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub file_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub page_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file_revision::Entity")]
    FileRevision,
    #[sea_orm(
        belongs_to = "super::page::Entity",
        from = "Column::PageId",
        to = "super::page::Column::PageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Page,
}

impl Related<super::file_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileRevision.def()
    }
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
