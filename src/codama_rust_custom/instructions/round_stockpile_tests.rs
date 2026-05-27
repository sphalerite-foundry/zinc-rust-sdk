use super::resolve_round_stockpile_id;

#[test]
fn resolves_active_stockpile_first() {
    assert_eq!(resolve_round_stockpile_id(Some(7), Some(8), 9), Some(7));
}

#[test]
fn omits_unresolved_stockpile_when_no_active_stockpile_exists() {
    assert_eq!(resolve_round_stockpile_id(None, Some(8), 9), None);
}

#[test]
fn omits_latest_initialized_stockpile_between_cycles() {
    assert_eq!(resolve_round_stockpile_id(None, None, 9), None);
}

#[test]
fn returns_none_before_any_stockpile_has_been_initialized() {
    assert_eq!(resolve_round_stockpile_id(None, None, 0), None);
}
