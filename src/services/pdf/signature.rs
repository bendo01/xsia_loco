use crate::models::academic::student::campaign::activities::data_objects::DataObject as ReferenceDataObject;
// use chrono::NaiveDateTime;
use loco_rs::prelude::*;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature;

impl Signature {
    pub async fn generate_student_signature(&self, ctx: &AppContext, activity_id: Uuid) -> String {
        // get activity
        let mut signature = String::from("Ditandatangani oleh mahasiswa");

        if let Ok(Some(activity)) = ReferenceDataObject::get_by_id(ctx, activity_id, true).await {
            if let Some(student) = &activity.student {
                signature = format!(
                    "Ditandatangani oleh mahasiswa: {} {}",
                    student.code, student.name
                );
            }
        }

        // Create with explicit UTF-8 encoding
        let code = QrCode::with_error_correction_level(signature, EcLevel::L).unwrap();
        let image = code
            .render()
            .min_dimensions(200, 200)
            .max_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        image
    }

    pub fn generate_course_department_signature(&self) -> String {
        let signature = String::from("Ditandatangani oleh kepala program studi");

        let code = QrCode::with_error_correction_level(signature, EcLevel::L).unwrap();
        let image = code
            .render()
            .min_dimensions(200, 200)
            .max_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        image
    }

    pub fn generate_academic_bureau_signature(&self) -> String {
        let signature =
            String::from("Ditandatangani oleh kepala Biro Administrasi Akademik dan Kemahasiswaan");

        let code = QrCode::with_error_correction_level(signature, EcLevel::L).unwrap();
        let image = code
            .render()
            .min_dimensions(200, 200)
            .max_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        image
    }
}
