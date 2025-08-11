use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_reference",
    table_name = "resign_statuses"
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
    Activities,
    Students,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Activities => Entity::has_many(AcademicStudentCampaignActivity::Entity)
                .from(Column::Id)
                .to(AcademicStudentCampaignActivity::Column::ResignStatusId)
                .into(),
            Self::Students => Entity::has_many(AcademicStudentMasterStudent::Entity)
                .from(Column::Id)
                .to(AcademicStudentMasterStudent::Column::ResignStatusId)
                .into(),
        }
    }
}

impl Related<AcademicStudentCampaignActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activities.def()
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}
