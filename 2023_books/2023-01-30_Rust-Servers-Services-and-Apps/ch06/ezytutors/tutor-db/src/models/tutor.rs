#![allow(dead_code)]

use crate::{response::Error, utils::update_from_option};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tutor {
    #[serde(default = "Tutor::default_tutor_id")]
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl Tutor {
    fn default_tutor_id() -> i32 {
        0
    }

    pub fn valid_for_create() -> Result<(), Error> {
        todo!()
    }

    pub fn update(&mut self, mut item: UpdateTutor) -> bool {
        update_from_option(&mut self.tutor_name, &mut item.tutor_name)
            || update_from_option(&mut self.tutor_pic_url, &mut item.tutor_pic_url)
            || update_from_option(&mut self.tutor_profile, &mut item.tutor_profile)
    }
}

impl UpdateTutor {
    pub fn valid_for_create() -> Result<(), Error> {
        todo!()
    }
}
