use crate::models::academic::campaign::transaction::class_codes::_entities::class_codes as AcademicCampaignTransactionClassCode;
use crate::models::academic::course::master::concentrations::_entities::concentrations as AcademicCourseMasterConcentration;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::reference::finances::_entities::finances as AcademicStudentReferenceFinance;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::academic::student::reference::resign_statuses::_entities::resign_statuses as AcademicStudentReferenceResignStatus;
use crate::models::academic::student::reference::selection_types::_entities::selection_types as AcademicStudentReferenceSelectionType;
use crate::models::academic::student::reference::statuses::_entities::statuses as AcademicStudentReferenceStatus;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_student_master", table_name = "students")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub nisn: Option<String>,
    pub name: String,
    pub registered: Date,
    pub individual_id: Uuid,
    pub unit_id: Uuid,
    pub academic_year_id: Uuid,
    pub curriculum_id: Uuid,
    pub class_code_id: Uuid,
    pub status_id: Uuid,
    pub registration_id: Uuid,
    pub resign_status_id: Uuid,
    pub concentration_id: Uuid,
    pub selection_type_id: Uuid,
    pub transfer_unit_id: Uuid,
    pub transfer_code: Option<String>,
    pub finance_fee: Option<f64>,
    pub finance_id: Option<Uuid>,
    pub feeder_id: Option<Uuid>,
    pub feeder_registration_id: Option<Uuid>,
    #[sea_orm(column_type = "Double", nullable)]
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Individual,
    Unit,
    AcademicYear,
    Curriculum,
    ClassCode,
    Status,
    Registration,
    ResignStatus,
    Concentration,
    SelectionType,
    // TransferUnit,
    Finance,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
            Self::Curriculum => Entity::belongs_to(AcademicCourseMasterCurriculum::Entity)
                .from(Column::CurriculumId)
                .to(AcademicCourseMasterCurriculum::Column::Id)
                .into(),
            Self::ClassCode => Entity::belongs_to(AcademicCampaignTransactionClassCode::Entity)
                .from(Column::ClassCodeId)
                .to(AcademicCampaignTransactionClassCode::Column::Id)
                .into(),
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Status => Entity::belongs_to(AcademicStudentReferenceStatus::Entity)
                .from(Column::StatusId)
                .to(AcademicStudentReferenceStatus::Column::Id)
                .into(),
            Self::Registration => Entity::belongs_to(AcademicStudentReferenceRegistration::Entity)
                .from(Column::RegistrationId)
                .to(AcademicStudentReferenceRegistration::Column::Id)
                .into(),
            Self::ResignStatus => Entity::belongs_to(AcademicStudentReferenceResignStatus::Entity)
                .from(Column::ResignStatusId)
                .to(AcademicStudentReferenceResignStatus::Column::Id)
                .into(),
            Self::Concentration => Entity::belongs_to(AcademicCourseMasterConcentration::Entity)
                .from(Column::ConcentrationId)
                .to(AcademicCourseMasterConcentration::Column::Id)
                .into(),
            Self::SelectionType => {
                Entity::belongs_to(AcademicStudentReferenceSelectionType::Entity)
                    .from(Column::SelectionTypeId)
                    .to(AcademicStudentReferenceSelectionType::Column::Id)
                    .into()
            }
            // Self::TransferUnit => Entity::belongs_to(InstitutionMasterUnit::Entity)
            //     .from(Column::TransferUnitId)
            //     .to(InstitutionMasterUnit::Column::Id)
            //     .into(),
            Self::Finance => Entity::belongs_to(AcademicStudentReferenceFinance::Entity)
                .from(Column::FinanceId)
                .to(AcademicStudentReferenceFinance::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl Related<AcademicCourseMasterCurriculum::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curriculum.def()
    }
}

impl Related<AcademicCampaignTransactionClassCode::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClassCode.def()
    }
}

impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicStudentReferenceStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Status.def()
    }
}

impl Related<AcademicStudentReferenceRegistration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Registration.def()
        // Relation::TransferUnit.def()
    }
}

impl Related<AcademicStudentReferenceResignStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResignStatus.def()
    }
}

impl Related<AcademicCourseMasterConcentration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Concentration.def()
    }
}

impl Related<AcademicStudentReferenceSelectionType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SelectionType.def()
    }
}

// impl Related<InstitutionMasterUnit::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::TransferUnit.def()
//     }
// }

impl Related<AcademicStudentReferenceFinance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Finance.def()
    }
}
