use crate::vram_fit::VramFit;

#[test]
fn flags_unknown_when_no_vram_declared() {
    assert_eq!(VramFit::assess(9.0, None), VramFit::Unknown);
}

#[test]
fn flags_comfortable_well_under_budget() {
    assert_eq!(VramFit::assess(8.0, Some(16.0)), VramFit::Comfortable);
}

#[test]
fn flags_tight_close_to_budget() {
    assert_eq!(VramFit::assess(13.0, Some(16.0)), VramFit::Tight);
}

#[test]
fn flags_too_large_over_budget() {
    assert_eq!(VramFit::assess(20.0, Some(16.0)), VramFit::TooLarge);
}
