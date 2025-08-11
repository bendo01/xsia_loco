use crate::models::academic::student::final_assignment::transaction::prerequisites::_entities::prerequisites as AcademicStudentFinalAssignmentTransactionPrerequisite;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_reference",
    table_name = "requirements"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub stage_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Prerequisites,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Prerequisites => Entity::has_many(
                AcademicStudentFinalAssignmentTransactionPrerequisite::Entity,
            )
            .from(Column::Id)
            .to(AcademicStudentFinalAssignmentTransactionPrerequisite::Column::RequirementId)
            .into(),
        }
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionPrerequisite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Prerequisites.def()
    }
}
