use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_reference",
    table_name = "categories"
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
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submissions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submissions => {
                Entity::has_many(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentFinalAssignmentTransactionSubmission::Column::CategoryId)
                    .into()
            }
        }
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSubmission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}
