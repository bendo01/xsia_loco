use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment_midtrans", table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub institution_id: Uuid,
    pub merchant_identification: String,
    pub client_key: String,
    pub server_key: String,
    pub sandbox_url: String,
    pub production_url: String,
    pub is_production: bool,
    pub is_sanitized: bool,
    pub is_3ds: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TransactionDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransactionDetails => Entity::has_many(PaymentMidtransTransactionDetail::Entity)
                .from(Column::Id)
                .to(PaymentMidtransTransactionDetail::Column::AccountId)
                .into(),
        }
    }
}

impl Related<PaymentMidtransTransactionDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionDetails.def()
    }
}
