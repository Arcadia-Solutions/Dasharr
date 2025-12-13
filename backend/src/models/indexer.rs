use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    models::user_stats::UserProfileScraped,
    services::user_stats::{
        aither::AitherScraper, anime_bytes::AnimeBytesScraper, anthelion::AnthelionScraper,
        blutopia::BlutopiaScraper, broadcasthenet::BroadcasthenetScraper,
        darkpeers::DarkPeersScraper, fear_no_peer::FearNoPeerScraper,
        gazelle_games::GazelleGamesScraper, homiehelpdesk::HomieHelpDeskScraper,
        ita_torrents::ItaTorrentsScraper, lst::LSTScraper, myanonamouse::MyAnonamouseScraper,
        oldtoons::OldToonsScraper, only_encodes::OnlyEncodesScraper, orpheus::OrpheusScraper,
        phoenix_project::PhoenixProjectScraper, racing4everyone::Racing4EveryoneScraper,
        rastastugan::RastastuganScraper, redacted::RedactedScraper, reel_flix::ReelFlixScraper,
        seed_pool::SeedPoolScraper, upload_cx::UploadCXScraper, yoinked::YoinkedScraper,
        yu_scene::YuSceneScraper,
    },
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthItem {
    pub value: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Indexer {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IndexerEnriched {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
    pub error: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub last_scraped_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatedIndexer {
    pub id: i32,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
}

#[async_trait]
pub trait Scraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped>;
}

impl Indexer {
    pub async fn scrape(self) -> Result<UserProfileScraped> {
        let scraper_ref: &dyn Scraper = match self.name.as_str() {
            "Redacted" => {
                static REDACTED_SCRAPER: RedactedScraper = RedactedScraper;
                &REDACTED_SCRAPER
            }
            "Orpheus" => {
                static ORPHEUS_SCRAPER: OrpheusScraper = OrpheusScraper;
                &ORPHEUS_SCRAPER
            }
            "GazelleGames" => {
                static GAZELLE_GAMES_SCRAPER: GazelleGamesScraper = GazelleGamesScraper;
                &GAZELLE_GAMES_SCRAPER
            }
            "Broadcasthenet" => {
                static BROADCASTHENET_SCRAPER: BroadcasthenetScraper = BroadcasthenetScraper;
                &BROADCASTHENET_SCRAPER
            }
            "Anthelion" => {
                static ANTHELION_SCRAPER: AnthelionScraper = AnthelionScraper;
                &ANTHELION_SCRAPER
            }
            "PhoenixProject" => {
                static PHOENIX_PROJECT_SCRAPER: PhoenixProjectScraper = PhoenixProjectScraper;
                &PHOENIX_PROJECT_SCRAPER
            }
            "AnimeBytes" => {
                static ANIME_BYTES_SCRAPER: AnimeBytesScraper = AnimeBytesScraper;
                &ANIME_BYTES_SCRAPER
            }
            "Blutopia" => {
                static BLUTOPIA_SCRAPER: BlutopiaScraper = BlutopiaScraper;
                &BLUTOPIA_SCRAPER
            }
            "Aither" => {
                static AITHER_SCRAPER: AitherScraper = AitherScraper;
                &AITHER_SCRAPER
            }
            "LST" => {
                static LST_SCRAPER: LSTScraper = LSTScraper;
                &LST_SCRAPER
            }
            "OldToons" => {
                static OLD_TOONS_SCRAPER: OldToonsScraper = OldToonsScraper;
                &OLD_TOONS_SCRAPER
            }
            "ReelFlix" => {
                static REEL_FLIX_SCRAPER: ReelFlixScraper = ReelFlixScraper;
                &REEL_FLIX_SCRAPER
            }
            "ItaTorrents" => {
                static ITA_TORRENTS_SCRAPER: ItaTorrentsScraper = ItaTorrentsScraper;
                &ITA_TORRENTS_SCRAPER
            }
            "OnlyEncodes" => {
                static ONLY_ENCODES_SCRAPER: OnlyEncodesScraper = OnlyEncodesScraper;
                &ONLY_ENCODES_SCRAPER
            }
            "SeedPool" => {
                static SEED_POOL_SCRAPER: SeedPoolScraper = SeedPoolScraper;
                &SEED_POOL_SCRAPER
            }
            "YuScene" => {
                static YU_SCENE_SCRAPER: YuSceneScraper = YuSceneScraper;
                &YU_SCENE_SCRAPER
            }
            "UploadCX" => {
                static UPLOAD_CX_SCRAPER: UploadCXScraper = UploadCXScraper;
                &UPLOAD_CX_SCRAPER
            }
            "FearNoPeer" => {
                static FEAR_NO_PEER_SCRAPER: FearNoPeerScraper = FearNoPeerScraper;
                &FEAR_NO_PEER_SCRAPER
            }
            "MyAnonamouse" => {
                static MY_ANONAMOUSE_SCRAPER: MyAnonamouseScraper = MyAnonamouseScraper;
                &MY_ANONAMOUSE_SCRAPER
            }
            "Yoinked" => {
                static YOINKED_SCRAPER: YoinkedScraper = YoinkedScraper;
                &YOINKED_SCRAPER
            }
            "DarkPeers" => {
                static DARKPEERS_SCRAPER: DarkPeersScraper = DarkPeersScraper;
                &DARKPEERS_SCRAPER
            }
            "Rastastugan" => {
                static RASTASTUGAN_SCRAPER: RastastuganScraper = RastastuganScraper;
                &RASTASTUGAN_SCRAPER
            }
            "HomieHelpDesk" => {
                static HOMIE_HELP_DESK_SCRAPER: HomieHelpDeskScraper = HomieHelpDeskScraper;
                &HOMIE_HELP_DESK_SCRAPER
            }
            "Racing4Everyone" => {
                static RACING_4_EVERYONE_SCRAPER: Racing4EveryoneScraper = Racing4EveryoneScraper;
                &RACING_4_EVERYONE_SCRAPER
            }
            _ => {
                return Err(Error::CouldNotScrapeIndexer(
                    "indexer has no scraper".into(),
                ));
            }
        };

        let profile = scraper_ref
            .scrape(self, &crate::services::HTTP_CLIENT)
            .await?;
        Ok(profile)
    }
}
