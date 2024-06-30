use crate::entity::PrefectureMaster;
use jisx0401::Prefecture;

pub struct PrefectureMasterRepository {}

impl PrefectureMasterRepository {
    pub async fn get(&self, prefecture: Prefecture) -> PrefectureMaster {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::get(&url).await?;
        let json = response.json::<PrefectureMaster>().await?;
        json
    }
}

impl PrefectureMasterRepository {
    pub fn get_blocking(&self, prefecture: Prefecture) -> PrefectureMaster {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::blocking::get(&url)?;
        let json = response.json::<PrefectureMaster>()?;
        json
    }
}
