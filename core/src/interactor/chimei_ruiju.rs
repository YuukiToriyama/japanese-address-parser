use crate::domain::chimei_ruiju::entity::{CityMaster, PrefectureMaster, TownMaster};
use crate::domain::chimei_ruiju::error::ApiError;
use crate::http::client::ApiClient;
use crate::repository::chimei_ruiju::city::CityMasterRepository;
use crate::repository::chimei_ruiju::prefecture::PrefectureMasterRepository;
use crate::repository::chimei_ruiju::town::TownMasterRepository;
use jisx0401::Prefecture;

pub(crate) trait ChimeiRuijuInteractor {
    /// 都道府県マスタを取得
    async fn get_prefecture_master(
        &self,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError>;
    /// 市区町村マスタを取得
    async fn get_city_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError>;
    /// 町名マスタを取得
    #[allow(dead_code)]
    async fn get_town_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError>;
}

pub(crate) struct ChimeiRuijuInteractorImpl<C: ApiClient> {
    prefecture_repository: PrefectureMasterRepository<C>,
    city_repository: CityMasterRepository<C>,
    town_repository: TownMasterRepository<C>,
}

impl<C: ApiClient> Default for ChimeiRuijuInteractorImpl<C> {
    fn default() -> Self {
        Self {
            prefecture_repository: PrefectureMasterRepository {
                api_client: C::new(),
            },
            city_repository: CityMasterRepository {
                api_client: C::new(),
            },
            town_repository: TownMasterRepository {
                api_client: C::new(),
            },
        }
    }
}

impl<C: ApiClient> ChimeiRuijuInteractor for ChimeiRuijuInteractorImpl<C> {
    async fn get_prefecture_master(
        &self,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError> {
        self.prefecture_repository.get(prefecture).await
    }

    async fn get_city_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        self.city_repository.get(prefecture, city_name).await
    }

    async fn get_town_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError> {
        self.town_repository
            .get(prefecture, city_name, town_name)
            .await
    }
}
