use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub events: Vec<Event>,
    #[serde(rename = "game_settings")]
    pub game_settings: GameSettings,
    pub phases: Vec<Phase>,
    pub teams: Vec<Team>,
    #[serde(rename = "total_players")]
    pub total_players: i64,
    pub elements: Vec<Element>,
    #[serde(rename = "element_stats")]
    pub element_stats: Vec<ElementStat>,
    #[serde(rename = "element_types")]
    pub element_types: Vec<ElementType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub name: String,
    #[serde(rename = "deadline_time")]
    pub deadline_time: String,
    #[serde(rename = "average_entry_score")]
    pub average_entry_score: i64,
    pub finished: bool,
    #[serde(rename = "data_checked")]
    pub data_checked: bool,
    #[serde(rename = "highest_scoring_entry")]
    pub highest_scoring_entry: Option<i64>,
    #[serde(rename = "deadline_time_epoch")]
    pub deadline_time_epoch: i64,
    #[serde(rename = "deadline_time_game_offset")]
    pub deadline_time_game_offset: i64,
    #[serde(rename = "highest_score")]
    pub highest_score: Option<i64>,
    #[serde(rename = "is_previous")]
    pub is_previous: bool,
    #[serde(rename = "is_current")]
    pub is_current: bool,
    #[serde(rename = "is_next")]
    pub is_next: bool,
    #[serde(rename = "chip_plays")]
    pub chip_plays: Vec<ChipPlay>,
    #[serde(rename = "most_selected")]
    pub most_selected: Option<i64>,
    #[serde(rename = "most_transferred_in")]
    pub most_transferred_in: Option<i64>,
    #[serde(rename = "top_element")]
    pub top_element: Option<i64>,
    #[serde(rename = "top_element_info")]
    pub top_element_info: Option<TopElementInfo>,
    #[serde(rename = "transfers_made")]
    pub transfers_made: i64,
    #[serde(rename = "most_captained")]
    pub most_captained: Option<i64>,
    #[serde(rename = "most_vice_captained")]
    pub most_vice_captained: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChipPlay {
    #[serde(rename = "chip_name")]
    pub chip_name: String,
    #[serde(rename = "num_played")]
    pub num_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopElementInfo {
    pub id: i64,
    pub points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    #[serde(rename = "league_join_private_max")]
    pub league_join_private_max: i64,
    #[serde(rename = "league_join_public_max")]
    pub league_join_public_max: i64,
    #[serde(rename = "league_max_size_public_classic")]
    pub league_max_size_public_classic: i64,
    #[serde(rename = "league_max_size_public_h2h")]
    pub league_max_size_public_h2_h: i64,
    #[serde(rename = "league_max_size_private_h2h")]
    pub league_max_size_private_h2_h: i64,
    #[serde(rename = "league_max_ko_rounds_private_h2h")]
    pub league_max_ko_rounds_private_h2_h: i64,
    #[serde(rename = "league_prefix_public")]
    pub league_prefix_public: String,
    #[serde(rename = "league_points_h2h_win")]
    pub league_points_h2_h_win: i64,
    #[serde(rename = "league_points_h2h_lose")]
    pub league_points_h2_h_lose: i64,
    #[serde(rename = "league_points_h2h_draw")]
    pub league_points_h2_h_draw: i64,
    #[serde(rename = "league_ko_first_instead_of_random")]
    pub league_ko_first_instead_of_random: bool,
    #[serde(rename = "cup_start_event_id")]
    pub cup_start_event_id: i64,
    #[serde(rename = "cup_stop_event_id")]
    pub cup_stop_event_id: i64,
    #[serde(rename = "cup_qualifying_method")]
    pub cup_qualifying_method: String,
    #[serde(rename = "cup_type")]
    pub cup_type: String,
    #[serde(rename = "squad_squadplay")]
    pub squad_squadplay: i64,
    #[serde(rename = "squad_squadsize")]
    pub squad_squadsize: i64,
    #[serde(rename = "squad_team_limit")]
    pub squad_team_limit: i64,
    #[serde(rename = "squad_total_spend")]
    pub squad_total_spend: i64,
    #[serde(rename = "ui_currency_multiplier")]
    pub ui_currency_multiplier: i64,
    #[serde(rename = "ui_use_special_shirts")]
    pub ui_use_special_shirts: bool,
    #[serde(rename = "ui_special_shirt_exclusions")]
    pub ui_special_shirt_exclusions: Vec<Option<String>>,
    #[serde(rename = "stats_form_days")]
    pub stats_form_days: i64,
    #[serde(rename = "sys_vice_captain_enabled")]
    pub sys_vice_captain_enabled: bool,
    #[serde(rename = "transfers_cap")]
    pub transfers_cap: i64,
    #[serde(rename = "transfers_sell_on_fee")]
    pub transfers_sell_on_fee: f64,
    #[serde(rename = "league_h2h_tiebreak_stats")]
    pub league_h2_h_tiebreak_stats: Vec<String>,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub id: i64,
    pub name: String,
    #[serde(rename = "start_event")]
    pub start_event: i64,
    #[serde(rename = "stop_event")]
    pub stop_event: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub code: i64,
    pub draw: i64,
    pub form: Option<String>,
    pub id: i64,
    pub loss: i64,
    pub name: String,
    pub played: i64,
    pub points: i64,
    pub position: i64,
    #[serde(rename = "short_name")]
    pub short_name: String,
    pub strength: i64,
    #[serde(rename = "team_division")]
    pub team_division: Option<String>,
    pub unavailable: bool,
    pub win: i64,
    #[serde(rename = "strength_overall_home")]
    pub strength_overall_home: i64,
    #[serde(rename = "strength_overall_away")]
    pub strength_overall_away: i64,
    #[serde(rename = "strength_attack_home")]
    pub strength_attack_home: i64,
    #[serde(rename = "strength_attack_away")]
    pub strength_attack_away: i64,
    #[serde(rename = "strength_defence_home")]
    pub strength_defence_home: i64,
    #[serde(rename = "strength_defence_away")]
    pub strength_defence_away: i64,
    #[serde(rename = "pulse_id")]
    pub pulse_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "chance_of_playing_next_round")]
    pub chance_of_playing_next_round: Option<i64>,
    #[serde(rename = "chance_of_playing_this_round")]
    pub chance_of_playing_this_round: Option<i64>,
    pub code: i64,
    #[serde(rename = "cost_change_event")]
    pub cost_change_event: i64,
    #[serde(rename = "cost_change_event_fall")]
    pub cost_change_event_fall: i64,
    #[serde(rename = "cost_change_start")]
    pub cost_change_start: i64,
    #[serde(rename = "cost_change_start_fall")]
    pub cost_change_start_fall: i64,
    #[serde(rename = "dreamteam_count")]
    pub dreamteam_count: i64,
    #[serde(rename = "element_type")]
    pub element_type: i64,
    #[serde(rename = "ep_next")]
    pub ep_next: String,
    #[serde(rename = "ep_this")]
    pub ep_this: String,
    #[serde(rename = "event_points")]
    pub event_points: i64,
    #[serde(rename = "first_name")]
    pub first_name: String,
    pub form: String,
    pub id: i64,
    #[serde(rename = "in_dreamteam")]
    pub in_dreamteam: bool,
    pub news: String,
    #[serde(rename = "news_added")]
    pub news_added: Option<String>,
    #[serde(rename = "now_cost")]
    pub now_cost: i64,
    pub photo: String,
    #[serde(rename = "points_per_game")]
    pub points_per_game: String,
    #[serde(rename = "second_name")]
    pub second_name: String,
    #[serde(rename = "selected_by_percent")]
    pub selected_by_percent: String,
    pub special: bool,
    #[serde(rename = "squad_number")]
    pub squad_number: Option<String>,
    pub status: String,
    pub team: i64,
    #[serde(rename = "team_code")]
    pub team_code: i64,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    #[serde(rename = "transfers_in")]
    pub transfers_in: i64,
    #[serde(rename = "transfers_in_event")]
    pub transfers_in_event: i64,
    #[serde(rename = "transfers_out")]
    pub transfers_out: i64,
    #[serde(rename = "transfers_out_event")]
    pub transfers_out_event: i64,
    #[serde(rename = "value_form")]
    pub value_form: String,
    #[serde(rename = "value_season")]
    pub value_season: String,
    #[serde(rename = "web_name")]
    pub web_name: String,
    pub minutes: i64,
    #[serde(rename = "goals_scored")]
    pub goals_scored: i64,
    pub assists: i64,
    #[serde(rename = "clean_sheets")]
    pub clean_sheets: i64,
    #[serde(rename = "goals_conceded")]
    pub goals_conceded: i64,
    #[serde(rename = "own_goals")]
    pub own_goals: i64,
    #[serde(rename = "penalties_saved")]
    pub penalties_saved: i64,
    #[serde(rename = "penalties_missed")]
    pub penalties_missed: i64,
    #[serde(rename = "yellow_cards")]
    pub yellow_cards: i64,
    #[serde(rename = "red_cards")]
    pub red_cards: i64,
    pub saves: i64,
    pub bonus: i64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    #[serde(rename = "ict_index")]
    pub ict_index: String,
    #[serde(rename = "influence_rank")]
    pub influence_rank: i64,
    #[serde(rename = "influence_rank_type")]
    pub influence_rank_type: i64,
    #[serde(rename = "creativity_rank")]
    pub creativity_rank: i64,
    #[serde(rename = "creativity_rank_type")]
    pub creativity_rank_type: i64,
    #[serde(rename = "threat_rank")]
    pub threat_rank: i64,
    #[serde(rename = "threat_rank_type")]
    pub threat_rank_type: i64,
    #[serde(rename = "ict_index_rank")]
    pub ict_index_rank: i64,
    #[serde(rename = "ict_index_rank_type")]
    pub ict_index_rank_type: i64,
    #[serde(rename = "corners_and_indirect_freekicks_order")]
    pub corners_and_indirect_freekicks_order: Option<i64>,
    #[serde(rename = "corners_and_indirect_freekicks_text")]
    pub corners_and_indirect_freekicks_text: String,
    #[serde(rename = "direct_freekicks_order")]
    pub direct_freekicks_order: Option<i64>,
    #[serde(rename = "direct_freekicks_text")]
    pub direct_freekicks_text: String,
    #[serde(rename = "penalties_order")]
    pub penalties_order: Option<i64>,
    #[serde(rename = "penalties_text")]
    pub penalties_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementStat {
    pub label: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementType {
    pub id: i64,
    #[serde(rename = "plural_name")]
    pub plural_name: String,
    #[serde(rename = "plural_name_short")]
    pub plural_name_short: String,
    #[serde(rename = "singular_name")]
    pub singular_name: String,
    #[serde(rename = "singular_name_short")]
    pub singular_name_short: String,
    #[serde(rename = "squad_select")]
    pub squad_select: i64,
    #[serde(rename = "squad_min_play")]
    pub squad_min_play: i64,
    #[serde(rename = "squad_max_play")]
    pub squad_max_play: i64,
    #[serde(rename = "ui_shirt_specific")]
    pub ui_shirt_specific: bool,
    #[serde(rename = "sub_positions_locked")]
    pub sub_positions_locked: Vec<i64>,
    #[serde(rename = "element_count")]
    pub element_count: i64,
}
