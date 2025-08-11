use crate::models::person::master::biodatas::_entities::biodatas as PersonMasterBiodata;
use crate::models::person::reference::blood_types::_entities::blood_types as PersonReferenceBloodType;
use crate::models::person::reference::eye_colors::_entities::eye_colors as PersonReferenceEyeColor;
use crate::models::person::reference::hair_colors::_entities::hair_colors as PersonReferenceHairColor;
use crate::models::person::reference::hair_types::_entities::hair_types as PersonReferenceHairType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiodataDataObject {
    #[serde(flatten)]
    pub biodata: PersonMasterBiodata::Model,
    pub blood_type: Option<PersonReferenceBloodType::Model>,
    pub eye_color: Option<PersonReferenceEyeColor::Model>,
    pub hair_color: Option<PersonReferenceHairColor::Model>,
    pub hair_type: Option<PersonReferenceHairType::Model>,
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::too_many_lines)]
impl BiodataDataObject {
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let biodata = PersonMasterBiodata::Entity::find()
            .filter(PersonMasterBiodata::Column::Id.eq(id))
            .filter(PersonMasterBiodata::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;

        if let Some(biodata) = biodata {
            let blood_type = biodata
                .find_related(PersonReferenceBloodType::Entity)
                .filter(PersonReferenceBloodType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let eye_color = biodata
                .find_related(PersonReferenceEyeColor::Entity)
                .filter(PersonReferenceEyeColor::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let hair_color = biodata
                .find_related(PersonReferenceHairColor::Entity)
                .filter(PersonReferenceHairColor::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let hair_type = biodata
                .find_related(PersonReferenceHairType::Entity)
                .filter(PersonReferenceHairType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                biodata,
                blood_type,
                eye_color,
                hair_color,
                hair_type,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_individual_id(
        ctx: &AppContext,
        individual_id: Uuid,
    ) -> Result<Option<Self>> {
        let biodata = PersonMasterBiodata::Entity::find()
            .filter(PersonMasterBiodata::Column::IndividualId.eq(individual_id))
            .filter(PersonMasterBiodata::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;

        if let Some(biodata) = biodata {
            let blood_type = biodata
                .find_related(PersonReferenceBloodType::Entity)
                .filter(PersonReferenceBloodType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let eye_color = biodata
                .find_related(PersonReferenceEyeColor::Entity)
                .filter(PersonReferenceEyeColor::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let hair_color = biodata
                .find_related(PersonReferenceHairColor::Entity)
                .filter(PersonReferenceHairColor::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let hair_type = biodata
                .find_related(PersonReferenceHairType::Entity)
                .filter(PersonReferenceHairType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                biodata,
                blood_type,
                eye_color,
                hair_color,
                hair_type,
            }))
        } else {
            Ok(None)
        }
    }
}
