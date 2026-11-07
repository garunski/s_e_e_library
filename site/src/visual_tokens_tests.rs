use s_e_e_gui_kit::components::{
    button::{button_classes, ButtonSize, ButtonVariant},
    layout::page_header::{PAGE_HEADER_DESCRIPTION_CLASS, PAGE_TITLE_CLASSES},
    page_structure::EDITORIAL_FRAME_CLASS,
};

#[test]
fn shell_uses_shared_page_and_frame_tokens() {
    assert!(PAGE_TITLE_CLASSES.contains("text-xl"));
    assert!(PAGE_TITLE_CLASSES.contains("font-semibold"));
    assert!(PAGE_HEADER_DESCRIPTION_CLASS.contains("text-xs"));
    assert!(EDITORIAL_FRAME_CLASS.contains("border-zinc-200/80"));
    assert!(EDITORIAL_FRAME_CLASS.contains("dark:border-zinc-700"));
}

#[test]
fn shell_buttons_use_sky_focus_outlines_in_both_themes() {
    let classes = button_classes(ButtonVariant::Primary, ButtonSize::Medium, false, None);
    assert!(classes.contains("focus-visible:outline-sky-600"));
    assert!(classes.contains("dark:focus-visible:outline-sky-300"));
}
