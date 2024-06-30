use crate::entity::PrefectureMaster;
use crate::error::ApiError;
use jisx0401::Prefecture;

pub struct PrefectureMasterRepository {}

impl PrefectureMasterRepository {
    pub async fn get(&self, prefecture: &Prefecture) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::get(&url)
            .await
            .map_err(|error| ApiError::Network {
                url: url.clone(),
                status_code: error.status().unwrap(),
            })?;
        let json = response
            .json::<PrefectureMaster>()
            .await
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}

impl PrefectureMasterRepository {
    pub fn get_blocking(&self, prefecture: Prefecture) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::blocking::get(&url).map_err(|error| ApiError::Network {
            url: url.clone(),
            status_code: error.status().unwrap(),
        })?;
        let json = response
            .json::<PrefectureMaster>()
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}
