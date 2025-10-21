use crate::models::person::reference::religions::_entities::religions as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Validator {
    #[validate(range(min = 1, message = "Kode wajib angka bilangan bulat"))]
    pub code: i32,

    #[validate(length(min = 1, message = "Kode Huruf tidak boleh kosong"))]
    pub alphabet_code: String,

    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub name: String,
}

impl Validatable for ReferenceModel::ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            code: self.code.as_ref().to_owned(),
            name: self.name.as_ref().to_owned(),
            alphabet_code: self.alphabet_code.as_ref().to_owned(),
        })
    }
}
