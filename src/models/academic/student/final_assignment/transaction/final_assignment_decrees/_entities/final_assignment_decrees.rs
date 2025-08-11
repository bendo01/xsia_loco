use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "final_assignment_decrees"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub decree_number: String,
    pub decree_date: Date,
    pub unit_id: Option<Uuid>,
    pub activity_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Unit,
    Activity,
    Staff,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Activity => Entity::belongs_to(AcademicCampaignTransactionActivity::Entity)
                .from(Column::ActivityId)
                .to(AcademicCampaignTransactionActivity::Column::Id)
                .into(),
            Self::Staff => Entity::belongs_to(InstitutionMasterStaff::Entity)
                .from(Column::StaffId)
                .to(InstitutionMasterStaff::Column::Id)
                .into(),
        }
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicCampaignTransactionActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<InstitutionMasterStaff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}
