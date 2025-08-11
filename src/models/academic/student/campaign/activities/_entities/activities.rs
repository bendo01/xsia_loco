use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionUnitActivity;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::academic::student::reference::finances::_entities::finances as AcademicStudentReferenceFinance;
use crate::models::academic::student::reference::resign_statuses::_entities::resign_statuses as AcademicStudentReferenceResignStatus;
use crate::models::academic::student::reference::statuses::_entities::statuses as AcademicStudentReferenceStatus;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use sea_orm::entity::prelude::*;
use sea_query::IntoCondition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_campaign",
    table_name = "student_activities"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Double")]
    pub cumulative_index: f64,
    #[sea_orm(column_type = "Double")]
    pub grand_cumulative_index: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub total_credit: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub grand_total_credit: f64,
    pub student_id: Uuid,
    pub unit_activity_id: Uuid,
    pub status_id: Uuid,
    pub resign_status_id: Uuid,
    pub unit_id: Uuid,
    pub is_lock: bool,
    pub feeder_id: Uuid,
    #[sea_orm(column_type = "Double", nullable)]
    pub finance_fee: f64,
    pub finance_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UnitActivity,
    Student,
    Status,
    ResignStatus,
    Unit,
    Finance,
    DetailActivities,
    Payments,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::UnitActivity => {
                Entity::belongs_to(AcademicCampaignTransactionUnitActivity::Entity)
                    .from(Column::UnitActivityId)
                    .to(AcademicCampaignTransactionUnitActivity::Column::Id)
                    .into()
            }
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
            Self::Status => Entity::belongs_to(AcademicStudentReferenceStatus::Entity)
                .from(Column::StatusId)
                .to(AcademicStudentReferenceStatus::Column::Id)
                .into(),
            Self::ResignStatus => Entity::belongs_to(AcademicStudentReferenceResignStatus::Entity)
                .from(Column::ResignStatusId)
                .to(AcademicStudentReferenceResignStatus::Column::Id)
                .into(),
            Self::Finance => Entity::belongs_to(AcademicStudentReferenceFinance::Entity)
                .from(Column::FinanceId)
                .to(AcademicStudentReferenceFinance::Column::Id)
                .into(),
            Self::DetailActivities => {
                Entity::has_many(AcademicStudentCampaignDetailActivity::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentCampaignDetailActivity::Column::ActivityId)
                    .into()
            }

            Self::Payments => Entity::has_many(PaymentMidtransTransactionDetail::Entity)
                .from(Column::Id)
                .to(PaymentMidtransTransactionDetail::Column::TransactionDetailableId)
                .on_condition(|_left, _right| {
                    PaymentMidtransTransactionDetail::Column::TransactionDetailableType
                        .eq("App\\Models\\cademic\\Student\\Campaign\\Activity")
                        .into_condition()
                })
                .into(),
        }
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicCampaignTransactionUnitActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UnitActivity.def()
    }
}
impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}
impl Related<AcademicStudentReferenceStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Status.def()
    }
}
impl Related<AcademicStudentReferenceResignStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResignStatus.def()
    }
}

impl Related<AcademicStudentReferenceFinance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Finance.def()
    }
}

impl Related<AcademicStudentCampaignDetailActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivities.def()
    }
}

impl Related<PaymentMidtransTransactionDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payments.def()
    }
}
