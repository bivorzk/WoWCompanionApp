use dioxus::prelude::*;

use crate::types::GearSlotData;

#[derive(Clone, Copy)]
pub(crate) struct GearInspectorStore {
    pub(crate) selected_slot: Signal<Option<GearSlotData>>,
    pub(crate) show_upgrade_finder: Signal<bool>,
}

pub(crate) fn use_gear_inspector_store() -> GearInspectorStore {
    let selected_slot = use_signal(|| None::<GearSlotData>);
    let show_upgrade_finder = use_signal(|| false);

    GearInspectorStore {
        selected_slot,
        show_upgrade_finder,
    }
}

#[derive(Clone, Copy)]
pub(crate) struct InsightPanelStore {
    pub(crate) active_tab: Signal<usize>,
    pub(crate) collapsed: Signal<bool>,
    pub(crate) pinned: Signal<bool>,
}

pub(crate) fn use_insight_panel_store(
    initial_tab: usize,
    initial_pinned: bool,
) -> InsightPanelStore {
    let active_tab = use_signal(move || initial_tab);
    let collapsed = use_signal(|| false);
    let pinned = use_signal(move || initial_pinned);

    InsightPanelStore {
        active_tab,
        collapsed,
        pinned,
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TabPanelStore {
    pub(crate) active_tab: Signal<usize>,
}

pub(crate) fn use_tab_panel_store(initial_tab: usize) -> TabPanelStore {
    let active_tab = use_signal(move || initial_tab);

    TabPanelStore { active_tab }
}