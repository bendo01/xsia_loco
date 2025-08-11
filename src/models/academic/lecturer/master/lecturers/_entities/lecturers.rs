use crate::models::academic::lecturer::reference::contracts::_entities::contracts as AcademicLecturerReferenceContract;
use crate::models::academic::lecturer::reference::groups::_entities::groups as AcademicLecturerReferenceGroup;
use crate::models::academic::lecturer::reference::ranks::_entities::ranks as AcademicLecturerReferenceRank;
use crate::models::academic::lecturer::reference::statuses::_entities::statuses as AcademicLecturerReferenceStatus;
use crate::models::academic::lecturer::transaction::academic_groups::_entities::academic_groups as AcademicLecturerTransactionAcademicGroup;
use crate::models::academic::lecturer::transaction::academic_ranks::_entities::academic_ranks as AcademicLecturerTransactionAcademicRank;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_lecturer_master", table_name = "lecturers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub individual_id: Uuid,
    pub institution_id: Option<Uuid>,
    pub alternative_code: Option<String>,
    pub accessor_number: Option<String>,
    pub identification_number: Option<String>,
    pub status_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
    pub rank_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub front_title: Option<String>,
    pub last_title: Option<String>,
    pub feeder_id: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Contract,
    Institution,
    Individual,
    Rank,
    Status,
    Group,
    AcademicGroups,
    AcademicRanks,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Contract => Entity::belongs_to(AcademicLecturerReferenceContract::Entity)
                .from(Column::ContractId)
                .to(AcademicLecturerReferenceContract::Column::Id)
                .into(),
            Self::Institution => Entity::belongs_to(InstitutionMasterInstitution::Entity)
                .from(Column::InstitutionId)
                .to(InstitutionMasterInstitution::Column::Id)
                .into(),
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::Rank => Entity::belongs_to(AcademicLecturerReferenceRank::Entity)
                .from(Column::RankId)
                .to(AcademicLecturerReferenceRank::Column::Id)
                .into(),
            Self::Status => Entity::belongs_to(AcademicLecturerReferenceStatus::Entity)
                .from(Column::StatusId)
                .to(AcademicLecturerReferenceStatus::Column::Id)
                .into(),
            Self::Group => Entity::belongs_to(AcademicLecturerReferenceGroup::Entity)
                .from(Column::GroupId)
                .to(AcademicLecturerReferenceGroup::Column::Id)
                .into(),
            Self::AcademicGroups => {
                Entity::has_many(AcademicLecturerTransactionAcademicGroup::Entity)
                    .from(Column::Id)
                    .to(AcademicLecturerTransactionAcademicGroup::Column::LecturerId)
                    .into()
            }
            Self::AcademicRanks => {
                Entity::has_many(AcademicLecturerTransactionAcademicRank::Entity)
                    .from(Column::Id)
                    .to(AcademicLecturerTransactionAcademicRank::Column::LecturerId)
                    .into()
            }
        }
    }
}

impl Related<AcademicLecturerReferenceContract::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contract.def()
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Institution.def()
    }
}

impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

impl Related<AcademicLecturerReferenceRank::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rank.def()
    }
}

impl Related<AcademicLecturerReferenceStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Status.def()
    }
}

impl Related<AcademicLecturerReferenceGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<AcademicLecturerTransactionAcademicGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicGroups.def()
    }
}

impl Related<AcademicLecturerTransactionAcademicRank::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicRanks.def()
    }
}
