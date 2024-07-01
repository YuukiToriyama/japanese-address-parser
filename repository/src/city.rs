use crate::entity::{CityMaster, PrefectureMaster};
use crate::error::ApiError;
use jisx0401::Prefecture;

pub struct CityMasterRepository {}

impl CityMasterRepository {
    pub async fn get(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        let response = reqwest::get(&url)
            .await
            .map_err(|error| ApiError::Network { url: url.clone() })?;
        let json = response
            .json::<CityMaster>()
            .await
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}

impl CityMasterRepository {
    pub fn get_blocking(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        let response =
            reqwest::blocking::get(&url).map_err(|error| ApiError::Network { url: url.clone() })?;
        let json = response
            .json::<CityMaster>()
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}
