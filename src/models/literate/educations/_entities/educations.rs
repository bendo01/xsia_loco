use crate::models::literate::categories::_entities::categories as LiterateCategory;
use crate::models::literate::groups::_entities::groups as LiterateGroup;
use crate::models::literate::levels::_entities::levels as LiterateLevel;
use crate::models::literate::varieties::_entities::varieties as LiterateVariety;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "literate", table_name = "educations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub abbreviation: String,
    pub name: String,
    pub level_id: Uuid,
    pub group_id: Uuid,
    pub category_id: Uuid,
    pub variety_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Category,
    Group,
    Level,
    Variety,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Category => Entity::belongs_to(LiterateCategory::Entity)
                .from(Column::CategoryId)
                .to(LiterateCategory::Column::Id)
                .into(),
            Self::Group => Entity::belongs_to(LiterateGroup::Entity)
                .from(Column::GroupId)
                .to(LiterateGroup::Column::Id)
                .into(),
            Self::Level => Entity::belongs_to(LiterateLevel::Entity)
                .from(Column::LevelId)
                .to(LiterateLevel::Column::Id)
                .into(),
            Self::Variety => Entity::belongs_to(LiterateVariety::Entity)
                .from(Column::VarietyId)
                .to(LiterateVariety::Column::Id)
                .into(),
        }
    }
}

impl Related<LiterateCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<LiterateGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<LiterateLevel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Level.def()
    }
}

impl Related<LiterateVariety::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variety.def()
    }
}
