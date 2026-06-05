mod companion_service;
mod live_companion_service;
mod presentation_service;
mod search_service;
mod theme_service;

pub(crate) use companion_service::CompanionService;
pub(crate) use live_companion_service::LiveCompanionService;
pub(crate) use presentation_service::{
	arena_map_icon, gear_art_class, gear_quality_label, gear_slot_icon, gear_upgrade_tip,
	gear_upgrade_track, metric_icon, metric_trend, mythic_dungeon_icon, parse_color,
	quality_color, sparkline_end_y, sparkline_points, spec_icon,
};
pub(crate) use search_service::{
	build_search_dropdown, load_search_history, parse_search, push_search_history,
	search_message, SearchHistoryEntry, SearchMessage, SearchMessageKind,
};
pub(crate) use theme_service::{theme_for_class, ThemeTokens};