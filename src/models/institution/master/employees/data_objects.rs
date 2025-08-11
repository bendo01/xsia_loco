use super::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmployeeDataObject {
    #[serde(flatten)]
    pub employee: InstitutionMasterEmployee::Model,
    pub institution: Option<InstitutionMasterInstitution::Model>,
    pub individual: Option<PersonMasterIndividual::Model>,
    pub staffes: Option<Vec<InstitutionMasterStaff::Model>>,
}

impl EmployeeDataObject {
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let employee = InstitutionMasterEmployee::Entity::find_by_id(id)
            .filter(InstitutionMasterEmployee::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(employee) = employee {
            let institution = employee
                .find_related(InstitutionMasterInstitution::Entity)
                .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let individual = employee
                .find_related(PersonMasterIndividual::Entity)
                .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let staffes = employee
                .find_related(InstitutionMasterStaff::Entity)
                .filter(InstitutionMasterStaff::Column::DeletedAt.is_null())
                .all(&ctx.db)
                .await?;
            Ok(Some(Self {
                employee,
                individual,
                institution,
                staffes: if staffes.is_empty() {
                    None
                } else {
                    Some(staffes)
                },
            }))
        } else {
            Ok(None)
        }
    }
}
