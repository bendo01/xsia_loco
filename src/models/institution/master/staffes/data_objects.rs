use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StaffDataObject {
    #[serde(flatten)]
    pub staff: InstitutionMasterStaff::Model,
    pub employee: Option<InstitutionMasterEmployee::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub position_type: Option<InstitutionReferencePositionType::Model>,
}

impl StaffDataObject {
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let staff = InstitutionMasterStaff::Entity::find_by_id(id)
            .filter(InstitutionMasterStaff::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(staff) = staff {
            let position_type = staff
                .find_related(InstitutionReferencePositionType::Entity)
                .filter(InstitutionReferencePositionType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let employee = staff
                .find_related(InstitutionMasterEmployee::Entity)
                .filter(InstitutionMasterEmployee::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let unit = staff
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                staff,
                employee,
                unit,
                position_type,
            }))
        } else {
            Ok(None)
        }
    }
}
