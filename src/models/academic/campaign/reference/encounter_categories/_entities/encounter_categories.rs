use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_reference",
    table_name = "encounter_categories"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Teaches,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Teaches => Entity::has_many(AcademicCampaignTransactionTeach::Entity)
                .from(Column::Id)
                .to(AcademicCampaignTransactionTeach::Column::EncounterCategoryId)
                .into(),
        }
    }
}

impl Related<AcademicCampaignTransactionTeach::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teaches.def()
    }
}
