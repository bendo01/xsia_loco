use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::lecturer::reference::contracts::_entities::contracts as AcademicLecturerReferenceContract;
use crate::models::academic::lecturer::reference::groups::_entities::groups as AcademicLecturerReferenceGroup;
use crate::models::academic::lecturer::reference::ranks::_entities::ranks as AcademicLecturerReferenceRank;
use crate::models::academic::lecturer::reference::statuses::_entities::statuses as AcademicLecturerReferenceStatus;
use crate::models::academic::lecturer::transaction::academic_groups::_entities::academic_groups as AcademicLecturerTransactionAcademicGroup;
use crate::models::academic::lecturer::transaction::academic_ranks::_entities::academic_ranks as AcademicLecturerTransactionAcademicRank;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use loco_rs::prelude::*;
// use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LecturerDataObject {
    // #[serde(flatten)]
    pub lecturer: AcademicLecturerMasterLecturer::Model,
    pub contract: Option<AcademicLecturerReferenceContract::Model>,
    pub rank: Option<AcademicLecturerReferenceRank::Model>,
    pub group: Option<AcademicLecturerReferenceGroup::Model>,
    pub status: Option<AcademicLecturerReferenceStatus::Model>,
    pub institution: Option<InstitutionMasterInstitution::Model>,
    pub individual: Option<PersonMasterIndividual::Model>,
    pub academic_groups: Option<Vec<AcademicLecturerTransactionAcademicGroup::Model>>,
    pub academic_ranks: Option<Vec<AcademicLecturerTransactionAcademicRank::Model>>,
}

impl LecturerDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let lecturer = AcademicLecturerMasterLecturer::Entity::find_by_id(id)
            .filter(AcademicLecturerMasterLecturer::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(lecturer) = lecturer {
            let contract = lecturer
                .find_related(AcademicLecturerReferenceContract::Entity)
                .filter(AcademicLecturerReferenceContract::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let rank = lecturer
                .find_related(AcademicLecturerReferenceRank::Entity)
                .filter(AcademicLecturerReferenceRank::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let group = lecturer
                .find_related(AcademicLecturerReferenceGroup::Entity)
                .filter(AcademicLecturerReferenceGroup::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let status = lecturer
                .find_related(AcademicLecturerReferenceStatus::Entity)
                .filter(AcademicLecturerReferenceStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let institution = lecturer
                .find_related(InstitutionMasterInstitution::Entity)
                .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let individual = lecturer
                .find_related(PersonMasterIndividual::Entity)
                .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let academic_groups = lecturer
                .find_related(AcademicLecturerTransactionAcademicGroup::Entity)
                .filter(AcademicLecturerTransactionAcademicGroup::Column::DeletedAt.is_null())
                .all(&ctx.db)
                .await?;
            let academic_ranks = lecturer
                .find_related(AcademicLecturerTransactionAcademicRank::Entity)
                .filter(AcademicLecturerTransactionAcademicRank::Column::DeletedAt.is_null())
                .all(&ctx.db)
                .await?;
            Ok(Some(Self {
                lecturer,
                contract,
                individual,
                rank,
                group,
                status,
                institution,
                academic_groups: if academic_groups.is_empty() {
                    None
                } else {
                    Some(academic_groups)
                },
                academic_ranks: if academic_ranks.is_empty() {
                    None
                } else {
                    Some(academic_ranks)
                },
            }))
        } else {
            Ok(None)
        }
    }
}
