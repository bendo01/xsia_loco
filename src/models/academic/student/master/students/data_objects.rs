use crate::models::academic::campaign::transaction::class_codes::_entities::class_codes as AcademicCampaignTransactionClassCode;
use crate::models::academic::course::master::concentrations::_entities::concentrations as AcademicCourseMasterConcentration;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::academic::student::reference::finances::_entities::finances as AcademicStudentReferenceFinance;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::academic::student::reference::resign_statuses::_entities::resign_statuses as AcademicStudentReferenceResignStatus;
use crate::models::academic::student::reference::selection_types::_entities::selection_types as AcademicStudentReferenceSelectionType;
use crate::models::academic::student::reference::statuses::_entities::statuses as AcademicStudentReferenceStatus;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct StudentDataObject {
    pub student: AcademicStudentMasterStudent::Model,
    pub individual: Option<PersonMasterIndividual::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub academic_year: Option<AcademicGeneralReferenceAcademicYear::Model>,
    pub curriculum: Option<AcademicCourseMasterCurriculum::Model>,
    pub class_code: Option<AcademicCampaignTransactionClassCode::Model>,
    pub status: Option<AcademicStudentReferenceStatus::Model>,
    pub registration: Option<AcademicStudentReferenceRegistration::Model>,
    pub resign_status: Option<AcademicStudentReferenceResignStatus::Model>,
    pub concentration: Option<AcademicCourseMasterConcentration::Model>,
    pub selection_type: Option<AcademicStudentReferenceSelectionType::Model>,
    pub finance: Option<AcademicStudentReferenceFinance::Model>,
}

impl StudentDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let student = AcademicStudentMasterStudent::Entity::find_by_id(id)
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(student) = student {
            let individual = student
                .find_related(PersonMasterIndividual::Entity)
                .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let unit = student
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let academic_year = student
                .find_related(AcademicGeneralReferenceAcademicYear::Entity)
                .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let curriculum = student
                .find_related(AcademicCourseMasterCurriculum::Entity)
                .filter(AcademicCourseMasterCurriculum::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let class_code = student
                .find_related(AcademicCampaignTransactionClassCode::Entity)
                .filter(AcademicCampaignTransactionClassCode::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let status = student
                .find_related(AcademicStudentReferenceStatus::Entity)
                .filter(AcademicStudentReferenceStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let registration = student
                .find_related(AcademicStudentReferenceRegistration::Entity)
                .filter(AcademicStudentReferenceRegistration::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let resign_status = student
                .find_related(AcademicStudentReferenceResignStatus::Entity)
                .filter(AcademicStudentReferenceResignStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let concentration = student
                .find_related(AcademicCourseMasterConcentration::Entity)
                .filter(AcademicCourseMasterConcentration::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let selection_type = student
                .find_related(AcademicStudentReferenceSelectionType::Entity)
                .filter(AcademicStudentReferenceSelectionType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let finance = student
                .find_related(AcademicStudentReferenceFinance::Entity)
                .filter(AcademicStudentReferenceFinance::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                student,
                individual,
                unit,
                academic_year,
                curriculum,
                class_code,
                status,
                registration,
                resign_status,
                concentration,
                selection_type,
                finance,
            }))
        } else {
            Ok(None)
        }
    }
}
