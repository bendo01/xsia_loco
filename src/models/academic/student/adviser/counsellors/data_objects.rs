use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::adviser::counsellors::_entities::counsellors as AcademicStudentAdviserCounsellor;
use crate::models::academic::student::adviser::decrees::_entities::decrees as AcademicStudentAdviserDecree;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct CounsellorDataObject {
    pub counsellor: AcademicStudentAdviserCounsellor::Model,
    pub decree: Option<AcademicStudentAdviserDecree::Model>,
    pub student: Option<AcademicStudentMasterStudent::Model>,
    pub lecturer: Option<AcademicLecturerMasterLecturer::Model>,
}

impl CounsellorDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let counsellor = AcademicStudentAdviserCounsellor::Entity::find_by_id(id)
            .filter(AcademicStudentAdviserCounsellor::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(counsellor) = counsellor {
            let decree = counsellor
                .find_related(AcademicStudentAdviserDecree::Entity)
                .filter(AcademicStudentAdviserDecree::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let student = counsellor
                .find_related(AcademicStudentMasterStudent::Entity)
                .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let lecturer = counsellor
                .find_related(AcademicLecturerMasterLecturer::Entity)
                .filter(AcademicLecturerMasterLecturer::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                counsellor,
                decree,
                student,
                lecturer,
            }))
        } else {
            Ok(None)
        }
    }
}
