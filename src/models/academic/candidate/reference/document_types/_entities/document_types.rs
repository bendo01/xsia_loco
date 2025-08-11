use crate::models::academic::candidate::transaction::documents::_entities::documents as AcademicCandidateTransactionDocument;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_candidate_reference",
    table_name = "document_types"
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
    Documents,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Documents => {
                Entity::has_many(AcademicCandidateTransactionDocument::Entity).into()
            }
        }
    }
}

impl Related<AcademicCandidateTransactionDocument::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Documents.def()
    }
}
