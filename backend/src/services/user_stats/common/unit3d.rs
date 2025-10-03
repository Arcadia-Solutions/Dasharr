use serde::Deserialize;

use crate::{models::user_stats::UserProfileScraped, services::helpers::bytes_from_size_string};

#[derive(Debug, Deserialize)]
pub struct UnitedResponse {
    pub group: String,
    pub uploaded: String,
    pub downloaded: String,
    pub ratio: String,
    pub seeding: i32,
    pub leeching: i32,
    pub seedbonus: String,
}

impl From<UnitedResponse> for UserProfileScraped {
    fn from(wrapper: UnitedResponse) -> Self {
        UserProfileScraped {
            uploaded: bytes_from_size_string(&wrapper.uploaded),
            downloaded: bytes_from_size_string(&wrapper.downloaded),
            seeding: Some(wrapper.seeding),
            leeching: Some(wrapper.leeching),
            class: wrapper.group,
            bonus_points: Some(wrapper.seedbonus.parse::<f64>().unwrap_or(0.0).round() as i64),
            ratio: wrapper.ratio.parse::<f32>().unwrap_or(0.0),
            ..Default::default()
        }
    }
}
