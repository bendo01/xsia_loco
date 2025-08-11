use crate::models::academic::campaign::transaction::teach_lecturers::_entities::teach_lecturers as AcademicCampaignTransactionTeachLecturer;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_reference",
    table_name = "evaluation_types"
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
    pub feeder_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicCampaignTransactionTeachLecturerers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicCampaignTransactionTeachLecturerers => {
                Entity::has_many(AcademicCampaignTransactionTeachLecturer::Entity)
                    .from(Column::Id)
                    .to(AcademicCampaignTransactionTeachLecturer::Column::EvaluationTypeId)
                    .into()
            }
        }
    }
}

impl Related<AcademicCampaignTransactionTeachLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionTeachLecturerers.def()
    }
}
