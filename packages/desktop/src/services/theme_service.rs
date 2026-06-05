#[derive(Clone, Copy)]
pub(crate) struct ThemeTokens {
    pub(crate) accent: &'static str,
    pub(crate) accent_muted: &'static str,
}

pub(crate) fn theme_for_class(class_name: &str) -> ThemeTokens {
    let (accent, accent_muted) = match class_name {
        "Death Knight" => ("#C41E3A", "rgba(196,30,58,0.15)"),
        "Paladin" => ("#F48CBA", "rgba(244,140,186,0.15)"),
        "Warrior" => ("#C69B3A", "rgba(198,155,58,0.15)"),
        "Mage" => ("#3FC7EB", "rgba(63,199,235,0.15)"),
        "Warlock" => ("#8788EE", "rgba(135,136,238,0.15)"),
        "Druid" => ("#FF7C0A", "rgba(255,124,10,0.15)"),
        "Hunter" => ("#AAD372", "rgba(170,211,114,0.15)"),
        "Shaman" => ("#0070DD", "rgba(0,112,221,0.15)"),
        "Rogue" => ("#FFF468", "rgba(255,244,104,0.15)"),
        "Priest" => ("#FFFFFF", "rgba(255,255,255,0.10)"),
        "Monk" => ("#00FF98", "rgba(0,255,152,0.15)"),
        "Demon Hunter" => ("#A330C9", "rgba(163,48,201,0.15)"),
        "Evoker" => ("#33937F", "rgba(51,147,127,0.15)"),
        _ => ("#9896A0", "rgba(152,150,160,0.15)"),
    };

    ThemeTokens {
        accent,
        accent_muted,
    }
}