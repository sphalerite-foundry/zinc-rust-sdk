/// Resolves the stockpile PDA id that round instructions can pass for validation.
pub fn resolve_round_stockpile_id(
    active_stockpile_id: Option<u64>,
    unresolved_stockpile_id: Option<u64>,
    next_stockpile_id: u64,
) -> Option<u64> {
    active_stockpile_id
        .or(unresolved_stockpile_id)
        .or_else(|| next_stockpile_id.checked_sub(1))
}

#[cfg(test)]
#[path = "round_stockpile_tests.rs"]
mod tests;
