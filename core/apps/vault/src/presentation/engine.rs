use crate::application::diaper::use_cases::delete::DeleteDiaperUseCase;
use crate::application::diaper::use_cases::list_by_range::{
    ListDiaperByRangeCommand, ListDiaperByRangeUseCase,
};
use crate::application::diaper::use_cases::log::{LogDiaperCommand, LogDiaperUseCase};
use crate::application::feed::use_cases::delete::DeleteFeedUseCase;
use crate::application::feed::use_cases::list_by_range::{
    ListFeedByRangeCommand, ListFeedByRangeUseCase,
};
use crate::application::feed::use_cases::log::{LogFeedCommand, LogFeedUseCase};
use crate::application::growth::use_cases::list_by_range::{
    ListGrowthByRangeCommand, ListGrowthByRangeUseCase,
};
use crate::application::growth::use_cases::log::{LogGrowthCommand, LogGrowthUseCase};
use crate::application::media::use_cases::list_metadata::ListMediaMetadataUseCase;
use crate::application::media::use_cases::read_decrypted::ReadDecryptedMediaUseCase;
use crate::application::media::use_cases::store_encrypted::{
    StoreEncryptedMediaCommand, StoreEncryptedMediaUseCase,
};
use crate::application::milestone::use_cases::create::{
    CreateMilestoneCommand, CreateMilestoneUseCase,
};
use crate::application::milestone::use_cases::delete::DeleteMilestoneUseCase;
use crate::application::milestone::use_cases::detail::DetailMilestoneUseCase;
use crate::application::milestone::use_cases::list::{
    ListMilestonesCommand, ListMilestonesUseCase,
};
use crate::application::sleep::use_cases::delete::DeleteSleepUseCase;
use crate::application::sleep::use_cases::list_by_range::{
    ListSleepByRangeCommand, ListSleepByRangeUseCase,
};
use crate::application::sleep::use_cases::log::{LogSleepCommand, LogSleepUseCase};
use crate::application::sync::use_cases::sync_all::SyncAllUseCase;
use crate::application::vault::use_cases::init_master_key::InitMasterKeyUseCase;
use crate::application::vault::use_cases::unlock::{UnlockCommand, UnlockUseCase};
use crate::domain::diaper::entity::DiaperType;
use crate::domain::feed::entity::FeedType;
use crate::infrastructure::crypto::chacha20_engine::ChaCha20Engine;
use crate::infrastructure::repository::sqlite_diaper::SqliteDiaperRepository;
use crate::infrastructure::repository::sqlite_feed::SqliteFeedRepository;
use crate::infrastructure::repository::sqlite_growth::SqliteGrowthRepository;
use crate::infrastructure::repository::sqlite_media::SqliteMediaRepository;
use crate::infrastructure::repository::sqlite_milestone::SqliteMilestoneRepository;
use crate::infrastructure::repository::sqlite_pool;
use crate::infrastructure::repository::sqlite_sleep::SqliteSleepRepository;
use crate::infrastructure::sync::http_sync_client::HttpSyncClient;
use crate::infrastructure::sync::sqlite_sync_repository::SqliteSyncRepository;
use crate::presentation::dto::{
    DiaperLogDto, DiaperTypeDto, FeedLogDto, FeedTypeDto, GrowthLogDto, MediaItemDto,
    MilestoneDto, SleepLogDto, SyncStatusDto,
};
use crate::presentation::error::FfiError;
use crate::presentation::mappers;
use chrono::{TimeZone, Utc};
use std::sync::Mutex;
use uuid::Uuid;
use zeroize::Zeroizing;
struct SyncConfig {
    server_url: String,
    api_key: String,
}
#[derive(uniffi::Object)]
pub struct VaultEngine {
    master_key: Mutex<Option<Zeroizing<Vec<u8>>>>,
    milestone_repo: SqliteMilestoneRepository,
    growth_repo: SqliteGrowthRepository,
    media_repo: SqliteMediaRepository,
    feed_repo: SqliteFeedRepository,
    sleep_repo: SqliteSleepRepository,
    diaper_repo: SqliteDiaperRepository,
    sync_repo: SqliteSyncRepository,
    crypto: ChaCha20Engine,
    storage_dir: String,
    sync_config: Mutex<Option<SyncConfig>>,
}
#[uniffi::export]
impl VaultEngine {
    #[uniffi::constructor]
    pub fn new(db_path: String, storage_dir: String) -> Result<Self, FfiError> {
        let pool = sqlite_pool::open(&db_path)
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?;
        Ok(Self {
            master_key: Mutex::new(None),
            milestone_repo: SqliteMilestoneRepository::new(pool.clone()),
            growth_repo: SqliteGrowthRepository::new(pool.clone()),
            media_repo: SqliteMediaRepository::new(pool.clone()),
            feed_repo: SqliteFeedRepository::new(pool.clone()),
            sleep_repo: SqliteSleepRepository::new(pool.clone()),
            diaper_repo: SqliteDiaperRepository::new(pool.clone()),
            sync_repo: SqliteSyncRepository::new(pool),
            crypto: ChaCha20Engine,
            storage_dir,
            sync_config: Mutex::new(None),
        })
    }
    pub fn engine_version(&self) -> String {
        format!("vault {}", env!("CARGO_PKG_VERSION"))
    }
    pub fn generate_master_key(&self) -> Result<Vec<u8>, FfiError> {
        InitMasterKeyUseCase::execute().map_err(FfiError::from)
    }
    pub fn unlock(&self, raw_key: Vec<u8>) -> Result<(), FfiError> {
        let key = UnlockUseCase::execute(UnlockCommand { raw_key }).map_err(FfiError::from)?;
        let mut guard = self
            .master_key
            .lock()
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?;
        *guard = Some(key);
        Ok(())
    }
    pub fn create_milestone(
        &self,
        title: String,
        description: String,
        occurred_at_millis: i64,
    ) -> Result<MilestoneDto, FfiError> {
        let occurred_at = Utc
            .timestamp_millis_opt(occurred_at_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid occurred_at".into() })?;
        let uc = CreateMilestoneUseCase::new(&self.milestone_repo);
        let m = uc
            .execute(CreateMilestoneCommand { title, description, occurred_at })
            .map_err(FfiError::from)?;
        Ok(mappers::milestone_to_dto(m))
    }
    pub fn list_milestones(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<MilestoneDto>, FfiError> {
        let uc = ListMilestonesUseCase::new(&self.milestone_repo);
        let items = uc
            .execute(ListMilestonesCommand { limit, offset })
            .map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::milestone_to_dto).collect())
    }
    pub fn get_milestone(&self, id: String) -> Result<MilestoneDto, FfiError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        let uc = DetailMilestoneUseCase::new(&self.milestone_repo);
        uc.execute(uuid).map(mappers::milestone_to_dto).map_err(FfiError::from)
    }
    pub fn delete_milestone(&self, id: String) -> Result<(), FfiError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        let uc = DeleteMilestoneUseCase::new(&self.milestone_repo);
        uc.execute(uuid).map_err(FfiError::from)
    }
    pub fn log_growth(
        &self,
        weight_grams: Option<u32>,
        height_mm: Option<u32>,
        notes: String,
        logged_at_millis: i64,
    ) -> Result<GrowthLogDto, FfiError> {
        let logged_at = Utc
            .timestamp_millis_opt(logged_at_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid logged_at".into() })?;
        let uc = LogGrowthUseCase::new(&self.growth_repo);
        let g = uc
            .execute(LogGrowthCommand { weight_grams, height_mm, notes, logged_at })
            .map_err(FfiError::from)?;
        Ok(mappers::growth_log_to_dto(g))
    }
    pub fn list_growth_by_range(
        &self,
        from_millis: i64,
        to_millis: i64,
    ) -> Result<Vec<GrowthLogDto>, FfiError> {
        let from = Utc
            .timestamp_millis_opt(from_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid from".into() })?;
        let to = Utc
            .timestamp_millis_opt(to_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid to".into() })?;
        let uc = ListGrowthByRangeUseCase::new(&self.growth_repo);
        let items = uc
            .execute(ListGrowthByRangeCommand { from, to })
            .map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::growth_log_to_dto).collect())
    }
    pub fn store_media(
        &self,
        title: String,
        plaintext_bytes: Vec<u8>,
    ) -> Result<MediaItemDto, FfiError> {
        let key = self.key()?;
        let uc = StoreEncryptedMediaUseCase::new(&self.media_repo, &self.crypto);
        let item = uc
            .execute(StoreEncryptedMediaCommand {
                title,
                plaintext_bytes,
                storage_dir: self.storage_dir.clone(),
                master_key: key.to_vec(),
            })
            .map_err(FfiError::from)?;
        Ok(mappers::media_item_to_dto(item))
    }
    pub fn read_media(&self, id: String) -> Result<Vec<u8>, FfiError> {
        let key = self.key()?;
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        let uc = ReadDecryptedMediaUseCase::new(&self.media_repo, &self.crypto);
        uc.execute(uuid, &key).map_err(FfiError::from)
    }
    pub fn list_media(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<MediaItemDto>, FfiError> {
        let uc = ListMediaMetadataUseCase::new(&self.media_repo);
        let items = uc.execute(limit, offset).map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::media_item_to_dto).collect())
    }
    pub fn log_feed(
        &self,
        feed_type: FeedTypeDto,
        amount_ml: Option<u32>,
        duration_minutes: Option<u32>,
        side: Option<String>,
        notes: String,
        logged_at_millis: i64,
    ) -> Result<FeedLogDto, FfiError> {
        let logged_at = Utc
            .timestamp_millis_opt(logged_at_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid logged_at".into() })?;
        let ft = match feed_type {
            FeedTypeDto::Breast => FeedType::Breast,
            FeedTypeDto::Bottle => FeedType::Bottle,
            FeedTypeDto::Solid => FeedType::Solid,
        };
        let uc = LogFeedUseCase::new(&self.feed_repo);
        let f = uc
            .execute(LogFeedCommand { feed_type: ft, amount_ml, duration_minutes, side, notes, logged_at })
            .map_err(FfiError::from)?;
        Ok(mappers::feed_log_to_dto(f))
    }
    pub fn list_feed_by_range(
        &self,
        from_millis: i64,
        to_millis: i64,
    ) -> Result<Vec<FeedLogDto>, FfiError> {
        let from = Utc
            .timestamp_millis_opt(from_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid from".into() })?;
        let to = Utc
            .timestamp_millis_opt(to_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid to".into() })?;
        let uc = ListFeedByRangeUseCase::new(&self.feed_repo);
        let items = uc.execute(ListFeedByRangeCommand { from, to }).map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::feed_log_to_dto).collect())
    }
    pub fn delete_feed(&self, id: String) -> Result<(), FfiError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        DeleteFeedUseCase::new(&self.feed_repo).execute(uuid).map_err(FfiError::from)
    }
    pub fn log_sleep(
        &self,
        start_time_millis: i64,
        end_time_millis: i64,
        notes: String,
    ) -> Result<SleepLogDto, FfiError> {
        let start_time = Utc
            .timestamp_millis_opt(start_time_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid start_time".into() })?;
        let end_time = Utc
            .timestamp_millis_opt(end_time_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid end_time".into() })?;
        let uc = LogSleepUseCase::new(&self.sleep_repo);
        let s = uc
            .execute(LogSleepCommand { start_time, end_time, notes })
            .map_err(FfiError::from)?;
        Ok(mappers::sleep_log_to_dto(s))
    }
    pub fn list_sleep_by_range(
        &self,
        from_millis: i64,
        to_millis: i64,
    ) -> Result<Vec<SleepLogDto>, FfiError> {
        let from = Utc
            .timestamp_millis_opt(from_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid from".into() })?;
        let to = Utc
            .timestamp_millis_opt(to_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid to".into() })?;
        let uc = ListSleepByRangeUseCase::new(&self.sleep_repo);
        let items = uc.execute(ListSleepByRangeCommand { from, to }).map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::sleep_log_to_dto).collect())
    }
    pub fn delete_sleep(&self, id: String) -> Result<(), FfiError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        DeleteSleepUseCase::new(&self.sleep_repo).execute(uuid).map_err(FfiError::from)
    }
    pub fn log_diaper(
        &self,
        diaper_type: DiaperTypeDto,
        notes: String,
        logged_at_millis: i64,
    ) -> Result<DiaperLogDto, FfiError> {
        let logged_at = Utc
            .timestamp_millis_opt(logged_at_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid logged_at".into() })?;
        let dt = match diaper_type {
            DiaperTypeDto::Wet => DiaperType::Wet,
            DiaperTypeDto::Dirty => DiaperType::Dirty,
            DiaperTypeDto::Both => DiaperType::Both,
        };
        let uc = LogDiaperUseCase::new(&self.diaper_repo);
        let d = uc
            .execute(LogDiaperCommand { diaper_type: dt, notes, logged_at })
            .map_err(FfiError::from)?;
        Ok(mappers::diaper_log_to_dto(d))
    }
    pub fn list_diaper_by_range(
        &self,
        from_millis: i64,
        to_millis: i64,
    ) -> Result<Vec<DiaperLogDto>, FfiError> {
        let from = Utc
            .timestamp_millis_opt(from_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid from".into() })?;
        let to = Utc
            .timestamp_millis_opt(to_millis)
            .single()
            .ok_or_else(|| FfiError::Validation { msg: "invalid to".into() })?;
        let uc = ListDiaperByRangeUseCase::new(&self.diaper_repo);
        let items = uc.execute(ListDiaperByRangeCommand { from, to }).map_err(FfiError::from)?;
        Ok(items.into_iter().map(mappers::diaper_log_to_dto).collect())
    }
    pub fn delete_diaper(&self, id: String) -> Result<(), FfiError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| FfiError::Validation { msg: "invalid id".into() })?;
        DeleteDiaperUseCase::new(&self.diaper_repo).execute(uuid).map_err(FfiError::from)
    }
    pub fn configure_sync_server(&self, server_url: String, api_key: String) -> Result<(), FfiError> {
        let mut guard = self
            .sync_config
            .lock()
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?;
        *guard = Some(SyncConfig { server_url, api_key });
        Ok(())
    }
    pub fn sync_now(&self) -> Result<SyncStatusDto, FfiError> {
        let guard = self
            .sync_config
            .lock()
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?;
        let config = guard
            .as_ref()
            .ok_or(FfiError::Validation { msg: "sync not configured".into() })?;
        let uc = SyncAllUseCase::new(&self.sync_repo, HttpSyncClient);
        let status = uc.execute(&config.server_url, &config.api_key).map_err(FfiError::from)?;
        Ok(sync_status_to_dto(status, true))
    }
    pub fn get_sync_status(&self) -> Result<SyncStatusDto, FfiError> {
        let is_configured = self
            .sync_config
            .lock()
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?
            .is_some();
        let uc = SyncAllUseCase::new(&self.sync_repo, HttpSyncClient);
        let status = uc.get_status().map_err(FfiError::from)?;
        Ok(sync_status_to_dto(status, is_configured))
    }
}
impl VaultEngine {
    fn key(&self) -> Result<Zeroizing<Vec<u8>>, FfiError> {
        self.master_key
            .lock()
            .map_err(|e| FfiError::Internal { msg: e.to_string() })?
            .as_ref()
            .cloned()
            .ok_or(FfiError::Validation {
                msg: "vault is locked — call unlock() first".into(),
            })
    }
}
fn sync_status_to_dto(s: crate::domain::sync::entity::SyncStatus, is_configured: bool) -> SyncStatusDto {
    SyncStatusDto {
        pending_milestones: s.pending_milestones,
        pending_growth_logs: s.pending_growth_logs,
        pending_media_items: s.pending_media_items,
        last_synced_at_millis: s.last_synced_at_millis,
        is_configured,
    }
}
