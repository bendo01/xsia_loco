use crate::models::academic::campaign::reference::evaluation_types::_entities::evaluation_types as AcademicCampaignReferenceEvaluationType;
use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_transaction",
    table_name = "teach_decrees"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: Option<String>,
    pub planning: i32,
    pub realization: i32,
    #[sea_orm(column_type = "Decimal(Some((3, 1)))", nullable)]
    pub credit: Option<Decimal>,
    pub is_lecturer_home_base: bool,
    pub lecturer_id: Uuid,
    pub teach_id: Uuid,
    pub evaluation_type_id: Uuid,
    pub feeder_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EvaluationType,
    Lecturer,
    Teach,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EvaluationType => {
                Entity::belongs_to(AcademicCampaignReferenceEvaluationType::Entity)
                    .from(Column::EvaluationTypeId)
                    .to(AcademicCampaignReferenceEvaluationType::Column::Id)
                    .into()
            }
            Self::Lecturer => Entity::belongs_to(AcademicLecturerMasterLecturer::Entity)
                .from(Column::LecturerId)
                .to(AcademicLecturerMasterLecturer::Column::Id)
                .into(),
            Self::Teach => Entity::belongs_to(AcademicCampaignTransactionTeach::Entity)
                .from(Column::TeachId)
                .to(AcademicCampaignTransactionTeach::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCampaignReferenceEvaluationType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluationType.def()
    }
}

impl Related<AcademicLecturerMasterLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lecturer.def()
    }
}

impl Related<AcademicCampaignTransactionTeach::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teach.def()
    }
}
