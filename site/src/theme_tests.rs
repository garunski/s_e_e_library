use s_e_e_shapes::Theme;

use crate::theme::{
    parse_theme_storage_label, theme_appears_dark, theme_storage_label, THEME_STORAGE_KEY,
};

#[test]
fn theme_storage_key_matches_app() {
    assert_eq!(THEME_STORAGE_KEY, "see.theme");
}

#[test]
fn theme_storage_labels_round_trip() {
    for theme in [Theme::Light, Theme::Dark, Theme::System] {
        let label = theme_storage_label(&theme);
        assert_eq!(parse_theme_storage_label(label), Some(theme));
    }
}

#[test]
fn theme_appears_dark_follows_system_for_system_theme() {
    assert!(theme_appears_dark(&Theme::System, true));
    assert!(!theme_appears_dark(&Theme::System, false));
    assert!(theme_appears_dark(&Theme::Dark, false));
    assert!(!theme_appears_dark(&Theme::Light, true));
}
