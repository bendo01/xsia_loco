use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment_midtrans", table_name = "item_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub price: i64,
    pub quantity: i32,
    pub transaction_detail_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TransactionDetail,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransactionDetail => Entity::belongs_to(PaymentMidtransTransactionDetail::Entity)
                .from(Column::TransactionDetailId)
                .to(PaymentMidtransTransactionDetail::Column::Id)
                .into(),
        }
    }
}

impl Related<PaymentMidtransTransactionDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionDetail.def()
    }
}
