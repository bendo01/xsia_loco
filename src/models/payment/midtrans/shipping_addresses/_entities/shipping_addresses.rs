use crate::models::payment::midtrans::customer_details::_entities::customer_details as PaymentMidtransCustomerDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "payment_midtrans", table_name = "shipping_addresses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub country_code: String,
    pub customer_detail_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CustomerDetail,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CustomerDetail => Entity::belongs_to(PaymentMidtransCustomerDetail::Entity)
                .from(Column::CustomerDetailId)
                .to(PaymentMidtransCustomerDetail::Column::Id)
                .into(),
        }
    }
}

impl Related<PaymentMidtransCustomerDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomerDetail.def()
    }
}
