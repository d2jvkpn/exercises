use crate::utils;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    #[serde(default = "Course::default_course_id")]
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<DateTime<Utc>>,
}

impl Course {
    fn default_course_id() -> i32 {
        0
    }

    pub fn update(&mut self, mut item: UpdateCourse) {
        self.course_name = item.course_name.unwrap_or(self.course_name.clone());

        utils::replace_option(&mut self.course_description, &mut item.course_description);

        utils::replace_option(&mut self.course_format, &mut item.course_format);

        utils::replace_option(&mut self.course_structure, &mut item.course_structure);

        utils::replace_option(&mut self.course_duration, &mut item.course_duration);

        utils::replace_option(&mut self.course_price, &mut item.course_price);

        utils::replace_option(&mut self.course_language, &mut item.course_language);

        utils::replace_option(&mut self.course_level, &mut item.course_level);
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}
