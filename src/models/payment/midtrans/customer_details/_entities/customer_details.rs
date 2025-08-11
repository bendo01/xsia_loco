use crate::models::payment::midtrans::billing_addresses::_entities::billing_addresses as PaymentMidtransBillingAddress;
use crate::models::payment::midtrans::shipping_addresses::_entities::shipping_addresses as PaymentMidtransShippingAddress;
use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment_midtrans", table_name = "customer_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
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
    BillingAddress,
    TransactionDetail,
    ShippingAddress,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransactionDetail => Entity::belongs_to(PaymentMidtransTransactionDetail::Entity)
                .from(Column::TransactionDetailId)
                .to(PaymentMidtransTransactionDetail::Column::Id)
                .into(),
            Self::BillingAddress => Entity::has_one(PaymentMidtransBillingAddress::Entity)
                .from(Column::Id)
                .to(PaymentMidtransBillingAddress::Column::CustomerDetailId)
                .into(),
            Self::ShippingAddress => Entity::has_one(PaymentMidtransShippingAddress::Entity)
                .from(Column::Id)
                .to(PaymentMidtransShippingAddress::Column::CustomerDetailId)
                .into(),
        }
    }
}

impl Related<PaymentMidtransTransactionDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionDetail.def()
    }
}

impl Related<PaymentMidtransBillingAddress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BillingAddress.def()
    }
}

impl Related<PaymentMidtransShippingAddress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShippingAddress.def()
    }
}
