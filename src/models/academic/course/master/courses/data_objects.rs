use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use crate::models::academic::course::reference::competences::_entities::competences as AcademicCourseReferenceCompetence;
use crate::models::academic::course::reference::groups::_entities::groups as AcademicCourseReferenceGroup;
use crate::models::academic::course::reference::varieties::_entities::varieties as AcademicCourseReferenceVariety;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct CourseDataObject {
    pub course: AcademicCourseMasterCourse::Model,
    pub group: Option<AcademicCourseReferenceGroup::Model>,
    pub variety: Option<AcademicCourseReferenceVariety::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub competence: Option<AcademicCourseReferenceCompetence::Model>,
    pub curriculum_details: Option<Vec<AcademicCourseMasterCurriculumDetail::Model>>,
}

impl CourseDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let course = AcademicCourseMasterCourse::Entity::find_by_id(id)
            .filter(AcademicCourseMasterCourse::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(course) = course {
            let group = course
                .find_related(AcademicCourseReferenceGroup::Entity)
                .filter(AcademicCourseReferenceGroup::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let variety = course
                .find_related(AcademicCourseReferenceVariety::Entity)
                .filter(AcademicCourseReferenceVariety::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let unit = course
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let competence = course
                .find_related(AcademicCourseReferenceCompetence::Entity)
                .filter(AcademicCourseReferenceCompetence::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let curriculum_details = if with_has_many_relationships {
                let curriculum_details_list = course
                    .find_related(AcademicCourseMasterCurriculumDetail::Entity)
                    .filter(AcademicCourseMasterCurriculumDetail::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if curriculum_details_list.is_empty() {
                    None
                } else {
                    Some(curriculum_details_list)
                }
            } else {
                None
            };

            Ok(Some(Self {
                course,
                group,
                variety,
                unit,
                competence,
                curriculum_details,
            }))
        } else {
            Ok(None)
        }
    }
}
