/// Resolves the active stockpile PDA id that round instructions can pass for validation.
pub fn resolve_round_stockpile_id(
    active_stockpile_id: Option<u64>,
    _unresolved_stockpile_id: Option<u64>,
    _next_stockpile_id: u64,
) -> Option<u64> {
    active_stockpile_id
}

#[cfg(test)]
#[path = "round_stockpile_tests.rs"]
mod tests;
