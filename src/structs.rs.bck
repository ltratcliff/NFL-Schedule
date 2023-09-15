use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Result;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub analytics: Analytics,
    pub ads: Ads,
    #[serde(rename = "DTCpackages")]
    pub dtcpackages: Dtcpackages,
    pub meta: Meta,
    pub now_feed_supported: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sport: Vec<String>,
    #[serde(rename = "tier2Nav")]
    pub tier2nav: Tier2Nav,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Analytics {
    pub metrics: Metrics,
    pub omniture: Omniture,
    pub chartbeat: Chartbeat,
    #[serde(rename = "ABTest")]
    pub abtest: Abtest,
    pub nielsen: Nielsen,
    pub device: String,
    pub is_feature_phone: bool,
    pub cto: bool,
    pub qualtrics: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    #[serde(rename = "nav_method")]
    pub nav_method: String,
    #[serde(rename = "page_url")]
    pub page_url: String,
    pub premium: bool,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "page_infrastructure")]
    pub page_infrastructure: String,
    #[serde(rename = "page_type")]
    pub page_type: String,
    pub league: String,
    #[serde(rename = "page_name")]
    pub page_name: String,
    pub section: String,
    pub sport: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Omniture {
    #[serde(rename = "espn3ContentType")]
    pub espn3content_type: String,
    pub league: String,
    pub country_region: String,
    pub nav_method: String,
    pub hier1: String,
    pub section: String,
    pub page_name: String,
    pub sections: String,
    pub site: String,
    pub premium: String,
    pub appearance: String,
    pub convr_sport: String,
    #[serde(rename = "pageURL")]
    pub page_url: String,
    pub lang: String,
    pub prop46: String,
    pub content_type: String,
    pub sport: String,
    pub account: String,
    pub site_type: String,
    pub prop58: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chartbeat {
    pub domain: String,
    pub sections: String,
    pub authors: String,
    pub path: String,
    pub title: String,
    pub zone: String,
    #[serde(rename = "loadPubJS")]
    pub load_pub_js: bool,
    #[serde(rename = "loadVidJS")]
    pub load_vid_js: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abtest {
    pub domain: String,
    pub environment: String,
    pub host: String,
    pub target: bool,
    pub optimizely: bool,
    pub t_script: String,
    pub o_script: String,
    pub is_targeted: bool,
    #[serde(rename = "targetURLs")]
    pub target_urls: Vec<TargetUrl>,
    #[serde(rename = "optimizelyURLs")]
    pub optimizely_urls: Vec<Value>,
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetUrl {
    pub regexp: String,
    pub site: String,
    pub flag: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nielsen {
    pub espnuk: Espnuk,
    pub espnau: Espnau,
    pub espn: Espn,
    pub fantasy: Fantasy,
    pub espndeportes: Espndeportes,
    pub espnfc: Espnfc,
    pub espnww: Espnww,
    pub general: General,
    pub espnza: Espnza,
    pub espnin: Espnin,
    pub watchespn: Watchespn,
    pub cricinfo: Cricinfo,
    pub espnbr: Espnbr,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnuk {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnau {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espn {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fantasy {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espndeportes {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnfc {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnww {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub ci: String,
    pub assetid: String,
    pub seg_b: String,
    pub sfcode: String,
    pub seg_a: String,
    pub section: String,
    pub seg_c: String,
    pub apn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnza {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnin {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchespn {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cricinfo {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espnbr {
    pub apid: String,
    pub vc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ads {
    #[serde(rename = "page_url")]
    pub page_url: String,
    pub prebid_ad_config: PrebidAdConfig,
    pub level: String,
    pub sizes_espn_plus: SizesEspnPlus,
    pub delay_in_page_ad_slots: bool,
    pub incontent_positions: IncontentPositions,
    pub show_espn_plus_ads: bool,
    pub kvps_espn_plus: Vec<KvpsEspnPlu>,
    pub network: String,
    pub refresh_on_breakpoint_change: bool,
    pub webview_override: WebviewOverride,
    pub sizes: Sizes,
    pub load: Load,
    pub betting_only_sizes: BettingOnlySizes,
    pub support_dynamic_page_load: bool,
    pub selector: String,
    pub whitelist_espn_plus: Vec<String>,
    pub disabled: String,
    #[serde(rename = "override")]
    pub override_field: Override,
    pub breakpoints: Breakpoints,
    pub dynamic_key_values: DynamicKeyValues,
    pub id: i64,
    pub kvps: Vec<Kvp>,
    pub base: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrebidAdConfig {
    pub use_prebid_bids: bool,
    pub timeout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizesEspnPlus {
    #[serde(rename = "banner-index")]
    pub banner_index: BannerIndex,
    pub gamecast: Gamecast,
    #[serde(rename = "banner-scoreboard")]
    pub banner_scoreboard: BannerScoreboard,
    pub banner: Banner,
    #[serde(rename = "incontent-betting")]
    pub incontent_betting: IncontentBetting,
    #[serde(rename = "native-betting")]
    pub native_betting: NativeBetting,
    pub instream: Instream,
    pub incontent: Incontent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerIndex {
    pub excluded_size: Vec<String>,
    pub mappings: Vec<Mapping>,
    pub default_size: Vec<i64>,
    pub excluded_profile: Vec<String>,
    pub included_countries: Vec<String>,
    pub pbjs: Pbjs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gamecast {
    pub mappings: Vec<Mapping2>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping2 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerScoreboard {
    pub excluded_size: Vec<String>,
    pub mappings: Vec<Mapping3>,
    pub default_size: Vec<i64>,
    pub included_countries: Vec<String>,
    pub pbjs: Pbjs2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping3 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs2 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub mappings: Vec<Mapping4>,
    pub default_size: Vec<i64>,
    pub pbjs: Pbjs3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping4 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs3 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncontentBetting {
    pub mappings: Vec<Mapping5>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping5 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeBetting {
    pub mappings: Vec<Mapping6>,
    pub default_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping6 {
    pub viewport: Vec<i64>,
    pub slot: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instream {
    pub mappings: Vec<Mapping7>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping7 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incontent {
    pub mappings: Vec<Mapping8>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping8 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncontentPositions {
    pub defaults: Defaults,
    pub index: Index,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    pub favorites: i64,
    pub news: i64,
    pub now: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub top: Top,
    pub nfl: Nfl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Top {
    pub favorites: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nfl {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KvpsEspnPlu {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebviewOverride {
    pub banner: Banner2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner2 {
    #[serde(rename = "mlb/stats")]
    pub mlb_stats: String,
    pub roster: String,
    #[serde(rename = "cfb/rankings")]
    pub cfb_rankings: String,
    #[serde(rename = "team/stats")]
    pub team_stats: String,
    #[serde(rename = "nba/stats")]
    pub nba_stats: String,
    #[serde(rename = "ncaaw/rankings")]
    pub ncaaw_rankings: String,
    #[serde(rename = "nfl/stats")]
    pub nfl_stats: String,
    pub standings: String,
    #[serde(rename = "cfb/stats")]
    pub cfb_stats: String,
    #[serde(rename = "ncb/rankings")]
    pub ncb_rankings: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes {
    pub gamecast: Gamecast2,
    pub overlay: Overlay,
    pub wallpaper: Wallpaper,
    #[serde(rename = "banner-scoreboard")]
    pub banner_scoreboard: BannerScoreboard2,
    pub incontent2: Incontent2,
    pub banner: Banner3,
    pub exclusions: Exclusions,
    #[serde(rename = "native-betting")]
    pub native_betting: NativeBetting2,
    #[serde(rename = "banner-index")]
    pub banner_index: BannerIndex2,
    #[serde(rename = "banner-webview")]
    pub banner_webview: BannerWebview,
    pub presby: Presby,
    pub presentedbylogo: Presentedbylogo,
    pub native: Native,
    pub incontentstrip: Incontentstrip,
    #[serde(rename = "incontent-betting")]
    pub incontent_betting: IncontentBetting2,
    pub instream: Instream2,
    pub incontentstrip2: Incontentstrip2,
    pub incontent: Incontent3,
    pub midpage: Midpage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gamecast2 {
    pub mappings: Vec<Mapping9>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping9 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    pub mappings: Vec<Mapping10>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping10 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallpaper {
    pub mappings: Vec<Mapping11>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping11 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerScoreboard2 {
    pub excluded_size: Vec<String>,
    pub mappings: Vec<Mapping12>,
    pub default_size: Vec<i64>,
    pub included_countries: Vec<String>,
    pub pbjs: Pbjs4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping12 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs4 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incontent2 {
    pub mappings: Vec<Mapping13>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping13 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner3 {
    pub mappings: Vec<Mapping14>,
    pub default_size: Vec<i64>,
    pub pbjs: Pbjs5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping14 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs5 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusions {
    pub mappings: Vec<Mapping15>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping15 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeBetting2 {
    pub mappings: Vec<Mapping16>,
    pub default_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping16 {
    pub viewport: Vec<i64>,
    pub slot: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerIndex2 {
    pub excluded_size: Vec<String>,
    pub mappings: Vec<Mapping17>,
    pub default_size: Vec<i64>,
    pub excluded_profile: Vec<String>,
    pub included_countries: Vec<String>,
    pub pbjs: Pbjs6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping17 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs6 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerWebview {
    pub excluded_size: Vec<String>,
    pub mappings: Vec<Mapping18>,
    pub default_size: Vec<i64>,
    pub included_countries: Vec<String>,
    pub pbjs: Pbjs7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping18 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pbjs7 {
    pub s: Vec<Vec<i64>>,
    pub xl: Vec<Vec<i64>>,
    pub l: Vec<Vec<i64>>,
    pub m: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presby {
    pub mappings: Vec<Mapping19>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping19 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presentedbylogo {
    pub mappings: Vec<Mapping20>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping20 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Native {
    pub mappings: Vec<Mapping21>,
    pub default_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping21 {
    pub viewport: Vec<i64>,
    pub slot: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incontentstrip {
    pub mappings: Vec<Mapping22>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping22 {
    pub viewport: Vec<i64>,
    pub slot: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncontentBetting2 {
    pub mappings: Vec<Mapping23>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping23 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instream2 {
    pub mappings: Vec<Mapping24>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping24 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incontentstrip2 {
    pub mappings: Vec<Mapping25>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping25 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incontent3 {
    pub mappings: Vec<Mapping26>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping26 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Midpage {
    pub mappings: Vec<Mapping27>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping27 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Load {
    pub schedule: Schedule,
    pub frontpage: Frontpage,
    pub defaults: Defaults2,
    pub index: Index2,
    pub scoreboard: Scoreboard,
    pub standings: Standings,
    pub story: Story,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frontpage {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defaults2 {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index2 {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scoreboard {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub tablet: String,
    pub desktop: String,
    pub mobile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BettingOnlySizes {
    #[serde(rename = "incontent-betting")]
    pub incontent_betting: IncontentBetting3,
    #[serde(rename = "native-betting")]
    pub native_betting: NativeBetting3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncontentBetting3 {
    pub mappings: Vec<Mapping28>,
    pub default_size: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping28 {
    pub viewport: Vec<i64>,
    pub slot: Vec<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeBetting3 {
    pub mappings: Vec<Mapping29>,
    pub default_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping29 {
    pub viewport: Vec<i64>,
    pub slot: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Override {
    pub banner: Banner4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner4 {
    pub preview: String,
    pub game: String,
    pub fightcenter: String,
    #[serde(rename = "match")]
    pub match_field: String,
    pub index: String,
    pub scoreboard: String,
    pub conversation: String,
    pub lineups: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoints {
    pub s: Vec<i64>,
    pub xl: Vec<i64>,
    pub l: Vec<i64>,
    pub m: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicKeyValues {
    pub profile: Profile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kvp {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dtcpackages {
    pub packages: Vec<Package>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub allows_restore: Option<bool>,
    pub upgrade: Option<Upgrade>,
    pub button_style: Option<String>,
    pub description: Option<String>,
    pub entitlement: String,
    pub subscription: Subscription,
    pub concurrency_limit: i64,
    pub logo_url: Option<String>,
    pub billing: Option<Billing2>,
    #[serde(rename = "isPPV")]
    pub is_ppv: bool,
    pub paywall: Paywall,
    pub country_codes: Vec<String>,
    pub is_iap: bool,
    #[serde(default)]
    pub accounts_hold: Vec<AccountsHold>,
    pub name: String,
    pub post_purchase_screen: Option<PostPurchaseScreen>,
    pub id: i64,
    pub short_name: Option<String>,
    pub sku: Option<String>,
    pub billing_disclaimer: Option<String>,
    pub show_pregame: bool,
    pub bundle: Option<Bundle>,
    pub voucher_code: Option<String>,
    pub campaign_code: Option<String>,
    pub price_increase: Option<Vec<PriceIncrease>>,
    pub subscriptions: Option<Vec<Subscription2>>,
    pub tune_in: Option<TuneIn>,
    pub trial: Option<Trial2>,
    pub trial_active: Option<bool>,
    pub embedded_paywall: Option<EmbeddedPaywall>,
    #[serde(rename = "isOOM")]
    pub is_oom: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade {
    pub required_entitlement: String,
    pub required_source: String,
    pub cta_button_subheader: String,
    pub subtitle: String,
    pub required_category_codes: Vec<String>,
    pub cta_button_title: String,
    pub cta_button_header: String,
    pub billing_disclaimer: String,
    pub voucher_code: String,
    pub disclaimer: String,
    pub billing: Billing,
    pub campaign_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing {
    pub show_check_box: bool,
    pub billing_disclosure_error: String,
    pub cta_button_title: String,
    pub billing_disclosure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub button_text_style: Option<String>,
    pub subscribe_text: String,
    pub subscribed_text: String,
    pub button_style: Option<String>,
    pub name: String,
    pub background_colors: Vec<String>,
    pub subscription_expired_text: String,
    pub description: String,
    pub bundled: Option<Bundled>,
    pub hero_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundled {
    pub manage_expired_text: String,
    pub manage_active_text: String,
    pub name: String,
    pub description: String,
    pub logo_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing2 {
    pub show_check_box: bool,
    pub billing_disclosure_error: String,
    pub cta_button_title: String,
    pub billing_disclosure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paywall {
    pub terms_of_use: Option<String>,
    pub cta_button_subheader: Option<String>,
    pub background_colors: Vec<String>,
    pub cta_button_style: Option<String>,
    pub subscriber_agreement_title: Option<String>,
    pub disclaimer: Option<String>,
    pub cta_button_text_style: Option<String>,
    pub legal_text1: Option<String>,
    pub terms_of_use_text: Option<String>,
    pub subscriber_agreement_text: Option<String>,
    pub cta_button_title: Option<String>,
    pub background_video_url: Option<String>,
    pub not_purchaseable_text: Option<String>,
    pub toggle: Option<Toggle>,
    pub privacy_policy_text: Option<String>,
    pub purchase_success_text: Option<String>,
    pub title: Option<String>,
    pub legal_text4: Option<String>,
    pub legal_text3: Option<String>,
    pub sponsor_image_url: Option<String>,
    pub legal_text2: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub enabled: bool,
    pub sections: Vec<Section>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub disclaimer_text2: String,
    pub is_default: bool,
    pub disclaimer_text1: String,
    pub notes: Vec<Note>,
    pub buttons: Vec<Button>,
    pub analytics_name: String,
    pub text: String,
    pub flag_text: Option<String>,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub image: String,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub trial_active: bool,
    pub analytics_name: String,
    pub sku: String,
    pub title: String,
    pub billing_disclaimer: String,
    pub text_color: String,
    pub disclaimer: String,
    pub trial: Trial,
    pub billing: Option<Billing3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trial {
    pub paywall: Paywall2,
    pub length: String,
    pub category_code: String,
    pub billing_disclaimer: String,
    pub voucher_code: String,
    pub campaign_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paywall2 {
    pub disclaimer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing3 {
    pub show_check_box: bool,
    pub billing_disclosure_error: String,
    pub cta_button_title: String,
    pub billing_disclosure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsHold {
    pub cta_url: String,
    pub refresh_entitlement_error_message: String,
    pub cta_text: String,
    pub network_error_message: String,
    pub cta_action: String,
    pub error_message: String,
    pub header: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub enabled: bool,
    pub sub_header: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPurchaseScreen {
    pub promo: Promo,
    pub button_text: String,
    pub message: String,
    pub enabled: bool,
    pub logo_url: String,
    pub background_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promo {
    pub link_url: String,
    pub link_text: String,
    pub message: String,
    pub enabled: bool,
    pub logo_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub required_entitlement: String,
    pub paywalls: Vec<Paywall3>,
    pub subtitle: String,
    pub post_purchase_screen: PostPurchaseScreen2,
    pub grace_period_seconds: i64,
    pub billing_disclaimer: String,
    pub enabled: bool,
    pub voucher_code: String,
    pub disclaimer: String,
    pub billing: Billing4,
    pub campaign_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paywall3 {
    pub legal_text1: String,
    pub disclaimer_text2: Option<String>,
    pub disclaimer_text1: Option<String>,
    pub buttons: Vec<Button2>,
    pub legal_text4: Option<String>,
    pub footer: Option<Footer>,
    pub legal_text3: Option<String>,
    pub legal_text2: Option<String>,
    pub cta_button_title: String,
    pub subscriber_agreement_title: String,
    pub title: Option<String>,
    pub hero_image_url: String,
    pub logo_url: String,
    pub terms_of_use: Option<String>,
    pub subtitle: String,
    pub background_colors: Vec<String>,
    pub not_purchaseable_text: String,
    pub secondary_title: Option<String>,
    pub cta_button_style: String,
    pub informative_login_text: String,
    pub disclaimer: String,
    pub order: i64,
    pub background_image_url: String,
    pub cta_button_text_style: String,
    pub header: Option<Header>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button2 {
    pub button_image: String,
    pub header_text: String,
    pub button_highlighted_image: String,
    pub title: String,
    pub subheader_text: Option<String>,
    pub text_color: String,
    pub disclaimer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Footer {
    pub subtitle: String,
    pub title: String,
    pub disclaimer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub subtitle: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPurchaseScreen2 {
    pub promo: Promo2,
    pub button_text: String,
    pub message: String,
    pub enabled: bool,
    pub logo_url: String,
    pub background_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promo2 {
    pub link_url: String,
    pub link_text: String,
    pub message: String,
    pub enabled: bool,
    pub logo_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing4 {
    pub show_check_box: bool,
    pub billing_disclosure_error: String,
    pub cta_button_title: String,
    pub billing_disclosure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceIncrease {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription2 {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TuneIn {
    pub button_text: String,
    pub upsell_href: String,
    pub upsell_text_key: String,
    pub overlay_img_url: String,
    pub background_img_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trial2 {
    pub paywall: Paywall4,
    pub length: String,
    pub category_code: String,
    pub billing_disclaimer: String,
    pub voucher_code: String,
    pub campaign_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paywall4 {
    pub cta_button_title: String,
    pub disclaimer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedPaywall {
    pub cta_button_title: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub image_width: i64,
    pub image: String,
    #[serde(rename = "twitter_card")]
    pub twitter_card: String,
    #[serde(rename = "og_site_name")]
    pub og_site_name: String,
    #[serde(rename = "twitter_app_id_iphone")]
    pub twitter_app_id_iphone: String,
    pub description: String,
    #[serde(rename = "og_type")]
    pub og_type: String,
    #[serde(rename = "twitter_app_name_googleplay")]
    pub twitter_app_name_googleplay: String,
    pub label: String,
    pub canonical: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub image_height: i64,
    #[serde(rename = "fb_app_id")]
    pub fb_app_id: String,
    #[serde(rename = "twitter_site")]
    pub twitter_site: String,
    pub root: String,
    #[serde(rename = "twitter_app_id_googleplay")]
    pub twitter_app_id_googleplay: String,
    #[serde(rename = "twitter_app_name_iphone")]
    pub twitter_app_name_iphone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tier2Nav {
    pub sub_nav_menu: SubNavMenu,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubNavMenu {
    pub navigation: Navigation,
    pub nav_id: i64,
    pub fallback: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Navigation {
    pub links: Vec<Link>,
    pub attributes: Attributes,
    pub id: i64,
    pub text: String,
    pub title: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub is_external: bool,
    pub short_text: String,
    pub rel: Vec<String>,
    pub text: String,
    pub href: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "sport_id")]
    pub sport_id: String,
    pub root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub links: Vec<Link2>,
    pub id: i64,
    pub title: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub items: Vec<Item2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link2 {
    pub is_external: bool,
    pub short_text: String,
    pub rel: Vec<String>,
    pub attributes: Option<Attributes2>,
    pub text: String,
    pub href: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes2 {
    pub mobile: Option<String>,
    pub route: Option<String>,
    pub placeholder: Option<String>,
    pub breakpoints: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "match-url")]
    pub match_url: Option<String>,
    pub sport_abbrev: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub links: Vec<Link3>,
    pub sort: Option<String>,
    pub id: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link3 {
    pub is_external: bool,
    pub short_text: String,
    #[serde(default)]
    pub rel: Vec<String>,
    pub text: String,
    pub is_premium: bool,
    pub href: String,
    pub lang: Option<String>,
    pub attributes: Option<Attributes3>,
    pub logo_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes3 {
    pub mobile: Option<String>,
    pub team_abbrev: Option<String>,
    pub team_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub schedule: HashMap<String, Games>,
    pub sorted_leagues: Vec<Value>,
    pub league: String,
    pub active_date: String,
    pub title: String,
    pub description: String,
    pub root: String,
    pub edition: String,
    pub page_title: String,
    pub days_to_show: i64,
    pub canonical: String,
    pub sport: String,
    pub parameters: Parameters,
    pub defaults: Defaults3,
    pub calendar: Vec<Calendar>,
    pub week_map: WeekMap,
    #[serde(rename = "og_type")]
    pub og_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub games: Vec<Game>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub date: String,
    pub uid: String,
    pub week: Week,
    pub name: String,
    pub competitions: Vec<Competition>,
    pub season: Season,
    pub links: Vec<Link6>,
    pub id: String,
    pub short_name: String,
    pub status: Status2,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub date: String,
    pub venue: Venue,
    pub conference_competition: bool,
    pub notes: Vec<Value>,
    pub time_valid: bool,
    pub geo_broadcasts: Vec<GeoBroadcast>,
    pub format: Format,
    pub broadcasts: Vec<Broadcast>,
    pub play_by_play_available: bool,
    pub leaders: Vec<Leader>,
    #[serde(rename = "type")]
    pub type_field: Type2,
    pub uid: String,
    pub competitors: Vec<Competitor>,
    pub headlines: Option<Vec<Headline>>,
    pub id: String,
    pub neutral_site: bool,
    pub recent: bool,
    pub attendance: i64,
    pub start_date: String,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub address: Address,
    pub full_name: String,
    pub indoor: bool,
    pub id: String,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoBroadcast {
    pub market: Market,
    pub media: Media,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub lang: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub id: String,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub regulation: Regulation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regulation {
    pub periods: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast {
    pub market: String,
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader {
    pub short_display_name: String,
    pub display_name: String,
    pub name: String,
    pub leaders: Vec<Leader2>,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader2 {
    pub display_value: String,
    pub athlete: Athlete,
    pub team: Team2,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Athlete {
    pub display_name: String,
    pub headshot: String,
    pub jersey: String,
    pub full_name: String,
    pub active: bool,
    pub links: Vec<Link4>,
    pub id: String,
    pub position: Position,
    pub team: Team,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link4 {
    pub rel: Vec<String>,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team2 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    pub id: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor {
    pub uid: String,
    pub home_away: String,
    pub score: String,
    pub winner: Option<bool>,
    pub records: Vec<Record>,
    pub id: String,
    pub team: Team3,
    #[serde(rename = "type")]
    pub type_field: String,
    pub linescores: Option<Vec<Linescore>>,
    pub order: i64,
    pub statistics: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub summary: String,
    pub name: String,
    pub abbreviation: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team3 {
    pub alternate_color: String,
    pub venue: Venue2,
    pub color: String,
    pub display_name: String,
    pub abbreviation: String,
    pub is_active: bool,
    pub short_display_name: String,
    pub uid: String,
    pub name: String,
    pub logo: String,
    pub location: String,
    pub links: Vec<Link5>,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue2 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link5 {
    pub is_external: bool,
    pub rel: Vec<String>,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub short_link_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type3 {
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub year: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link6 {
    pub is_external: bool,
    pub short_text: String,
    pub rel: Vec<String>,
    pub language: String,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status2 {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type4 {
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week2 {
    pub number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition2 {
    pub date: String,
    pub venue: Venue3,
    pub conference_competition: bool,
    pub notes: Vec<Value>,
    pub time_valid: bool,
    pub geo_broadcasts: Vec<GeoBroadcast2>,
    pub format: Format2,
    pub broadcasts: Vec<Broadcast2>,
    pub play_by_play_available: bool,
    pub leaders: Vec<Leader3>,
    #[serde(rename = "type")]
    pub type_field: Type6,
    pub uid: String,
    pub competitors: Vec<Competitor2>,
    pub id: String,
    pub neutral_site: bool,
    pub recent: bool,
    pub attendance: i64,
    pub start_date: String,
    pub status: Status3,
    #[serde(default)]
    pub headlines: Vec<Headline2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue3 {
    pub address: Address2,
    pub full_name: String,
    pub indoor: bool,
    pub id: String,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address2 {
    pub city: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoBroadcast2 {
    pub market: Market2,
    pub media: Media2,
    #[serde(rename = "type")]
    pub type_field: Type5,
    pub lang: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market2 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media2 {
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type5 {
    pub id: String,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format2 {
    pub regulation: Regulation2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regulation2 {
    pub periods: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast2 {
    pub market: String,
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader3 {
    pub short_display_name: String,
    pub display_name: String,
    pub name: String,
    pub leaders: Vec<Leader4>,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader4 {
    pub display_value: String,
    pub athlete: Athlete2,
    pub team: Team5,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Athlete2 {
    pub display_name: String,
    pub headshot: String,
    pub jersey: String,
    pub full_name: String,
    pub active: bool,
    pub links: Vec<Link7>,
    pub id: String,
    pub position: Position2,
    pub team: Team4,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link7 {
    pub rel: Vec<String>,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team4 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team5 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type6 {
    pub id: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor2 {
    pub uid: String,
    pub home_away: String,
    pub score: String,
    pub winner: bool,
    pub records: Vec<Record2>,
    pub id: String,
    pub team: Team6,
    #[serde(rename = "type")]
    pub type_field: String,
    pub linescores: Vec<Linescore2>,
    pub order: i64,
    pub statistics: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record2 {
    pub summary: String,
    pub name: String,
    pub abbreviation: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team6 {
    pub alternate_color: String,
    pub venue: Venue4,
    pub color: String,
    pub display_name: String,
    pub abbreviation: String,
    pub is_active: bool,
    pub short_display_name: String,
    pub uid: String,
    pub name: String,
    pub logo: String,
    pub location: String,
    pub links: Vec<Link8>,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue4 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link8 {
    pub is_external: bool,
    pub rel: Vec<String>,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore2 {
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status3 {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type7 {
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline2 {
    pub description: String,
    pub video: Vec<Video>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub short_link_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub duration: i64,
    pub device_restrictions: DeviceRestrictions,
    pub thumbnail: String,
    pub links: Links,
    pub id: i64,
    pub source: String,
    pub headline: String,
    pub tracking: Tracking,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRestrictions {
    pub devices: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub web: Web,
    pub mobile: Mobile,
    pub api: Api,
    pub source: Source2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web {
    pub short: Short,
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Short {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mobile {
    pub streaming: Streaming,
    pub alert: Alert,
    pub progressive_download: ProgressiveDownload,
    pub source: Source,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Streaming {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressiveDownload {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    #[serde(rename = "self")]
    pub self_field: Self_field2,
    pub artwork: Artwork,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub mezzanine: Mezzanine,
    pub hds: Hds,
    pub href: String,
    #[serde(rename = "HD")]
    pub hd: Hd,
    #[serde(rename = "HLS")]
    pub hls: Hls,
    pub flash: Flash,
    pub full: Full,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mezzanine {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hds {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hls {
    pub href: String,
    #[serde(rename = "HD")]
    pub hd: Hd2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flash {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking {
    pub league_name: String,
    pub tracking_name: String,
    pub coverage_type: String,
    pub sport_name: String,
    pub tracking_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season2 {
    pub year: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link9 {
    pub is_external: bool,
    pub short_text: String,
    pub rel: Vec<String>,
    pub language: String,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status4 {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type8 {
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week3 {
    pub number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition3 {
    pub date: String,
    pub venue: Venue5,
    pub conference_competition: bool,
    pub notes: Vec<Value>,
    pub time_valid: bool,
    pub geo_broadcasts: Vec<GeoBroadcast3>,
    pub format: Format3,
    pub broadcasts: Vec<Broadcast3>,
    pub play_by_play_available: bool,
    pub leaders: Vec<Leader5>,
    #[serde(rename = "type")]
    pub type_field: Type10,
    pub uid: String,
    pub competitors: Vec<Competitor3>,
    pub headlines: Vec<Headline3>,
    pub id: String,
    pub neutral_site: bool,
    pub recent: bool,
    pub attendance: i64,
    pub start_date: String,
    pub status: Status5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue5 {
    pub address: Address3,
    pub full_name: String,
    pub indoor: bool,
    pub id: String,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address3 {
    pub city: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoBroadcast3 {
    pub market: Market3,
    pub media: Media3,
    #[serde(rename = "type")]
    pub type_field: Type9,
    pub lang: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market3 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media3 {
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type9 {
    pub id: String,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format3 {
    pub regulation: Regulation3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regulation3 {
    pub periods: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast3 {
    pub market: String,
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader5 {
    pub short_display_name: String,
    pub display_name: String,
    pub name: String,
    pub leaders: Vec<Leader6>,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader6 {
    pub display_value: String,
    pub athlete: Athlete3,
    pub team: Team8,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Athlete3 {
    pub display_name: String,
    pub headshot: String,
    pub jersey: String,
    pub full_name: String,
    pub active: bool,
    pub links: Vec<Link10>,
    pub id: String,
    pub position: Position3,
    pub team: Team7,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link10 {
    pub rel: Vec<String>,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position3 {
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team7 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team8 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type10 {
    pub id: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor3 {
    pub uid: String,
    pub home_away: String,
    pub score: String,
    pub winner: Option<bool>,
    pub records: Vec<Record3>,
    pub id: String,
    pub team: Team9,
    #[serde(rename = "type")]
    pub type_field: String,
    pub linescores: Vec<Linescore3>,
    pub order: i64,
    pub statistics: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record3 {
    pub summary: String,
    pub name: String,
    pub abbreviation: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team9 {
    pub alternate_color: String,
    pub venue: Venue6,
    pub color: String,
    pub display_name: String,
    pub abbreviation: String,
    pub is_active: bool,
    pub short_display_name: String,
    pub uid: String,
    pub name: String,
    pub logo: String,
    pub location: String,
    pub links: Vec<Link11>,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue6 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link11 {
    pub is_external: bool,
    pub rel: Vec<String>,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore3 {
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline3 {
    pub description: String,
    pub video: Vec<Video2>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub short_link_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video2 {
    pub duration: i64,
    pub device_restrictions: DeviceRestrictions2,
    pub thumbnail: String,
    pub links: Links2,
    pub id: i64,
    pub source: String,
    pub headline: String,
    pub tracking: Tracking2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRestrictions2 {
    pub devices: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    pub web: Web2,
    pub mobile: Mobile2,
    pub api: Api2,
    pub source: Source4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web2 {
    pub short: Short2,
    #[serde(rename = "self")]
    pub self_field: Self_field3,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Short2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mobile2 {
    pub streaming: Streaming2,
    pub alert: Alert2,
    pub progressive_download: ProgressiveDownload2,
    pub source: Source3,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Streaming2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressiveDownload2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api2 {
    #[serde(rename = "self")]
    pub self_field: Self_field4,
    pub artwork: Artwork2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field4 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artwork2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source4 {
    pub mezzanine: Mezzanine2,
    pub hds: Hds2,
    pub href: String,
    #[serde(rename = "HD")]
    pub hd: Hd3,
    #[serde(rename = "HLS")]
    pub hls: Hls2,
    pub flash: Flash2,
    pub full: Full2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mezzanine2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hds2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hls2 {
    pub href: String,
    #[serde(rename = "HD")]
    pub hd: Hd4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd4 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flash2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Full2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking2 {
    pub league_name: String,
    pub tracking_name: String,
    pub coverage_type: String,
    pub sport_name: String,
    pub tracking_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status5 {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type11 {
    pub alt_detail: String,
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season3 {
    pub year: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link12 {
    pub is_external: bool,
    pub short_text: String,
    pub rel: Vec<String>,
    pub language: String,
    pub href: String,
    pub text: String,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status6 {
    pub period: i64,
    pub display_clock: String,
    pub clock: i64,
    #[serde(rename = "type")]
    pub type_field: Type12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type12 {
    pub alt_detail: String,
    pub name: String,
    pub description: String,
    pub id: String,
    pub state: String,
    pub completed: bool,
    pub detail: String,
    pub short_detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub week: i64,
    pub year: i64,
    pub seasontype: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defaults3 {
    pub week: i64,
    pub year: i64,
    pub seasontype: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub entries: Vec<Entry>,
    pub end_date: String,
    pub label: String,
    pub value: String,
    pub start_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub end_date: String,
    pub alternate_label: String,
    pub label: String,
    pub detail: String,
    pub value: String,
    pub start_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekMap {
    pub number: i64,
}
