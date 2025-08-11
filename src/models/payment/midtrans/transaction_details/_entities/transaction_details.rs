use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::payment::midtrans::accounts::_entities::accounts as PaymentMidtransAccount;
use crate::models::payment::midtrans::customer_details::_entities::customer_details as PaymentMidtransCustomerDetail;
use crate::models::payment::midtrans::item_details::_entities::item_details as PaymentMidtransItemDetail;
use sea_orm::entity::prelude::*;
use sea_query::IntoCondition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment_midtrans", table_name = "transaction_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub order_id: String,
    pub gross_amount: f64,
    pub account_id: Uuid,
    pub transaction_detailable_id: Uuid,
    pub transaction_detailable_type: String,
    pub is_paid: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Account,
    ItemDetails,
    CustomerDetail,
    StudentPayment,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Account => Entity::belongs_to(PaymentMidtransAccount::Entity)
                .from(Column::AccountId)
                .to(PaymentMidtransAccount::Column::Id)
                .into(),
            Self::StudentPayment => Entity::belongs_to(AcademicStudentCampaignActivity::Entity)
                .from(Column::TransactionDetailableId)
                .to(AcademicStudentCampaignActivity::Column::Id)
                .on_condition(|_left, _right| {
                    Column::TransactionDetailableType
                        .eq("App\\Models\\cademic\\Student\\Campaign\\Activity")
                        .into_condition()
                })
                .into(),
            Self::CustomerDetail => Entity::has_one(PaymentMidtransCustomerDetail::Entity)
                .from(Column::Id)
                .to(PaymentMidtransCustomerDetail::Column::TransactionDetailId)
                .into(),
            Self::ItemDetails => Entity::has_many(PaymentMidtransItemDetail::Entity)
                .from(Column::Id)
                .to(PaymentMidtransItemDetail::Column::TransactionDetailId)
                .into(),
        }
    }
}

impl Related<PaymentMidtransAccount::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<PaymentMidtransCustomerDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomerDetail.def()
    }
}

impl Related<PaymentMidtransItemDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemDetails.def()
    }
}

impl Related<AcademicStudentCampaignActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StudentPayment.def()
    }
}
