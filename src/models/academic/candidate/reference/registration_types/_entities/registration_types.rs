use crate::models::academic::candidate::reference::registration_categories::_entities::registration_categories as AcademicCandidateReferenceRegistrationCategory;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_candidate_reference",
    table_name = "registration_types"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub student_registration_id: Uuid,
    pub unit_id: Uuid,
    pub institution_id: Uuid,
    pub registration_category_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Registration,
    Unit,
    Category,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Registration => Entity::belongs_to(AcademicStudentReferenceRegistration::Entity)
                .from(Column::StudentRegistrationId)
                .to(AcademicStudentReferenceRegistration::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Category => {
                Entity::belongs_to(AcademicCandidateReferenceRegistrationCategory::Entity)
                    .from(Column::RegistrationCategoryId)
                    .to(AcademicCandidateReferenceRegistrationCategory::Column::Id)
                    .into()
            }
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicStudentReferenceRegistration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Registration.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicCandidateReferenceRegistrationCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}
