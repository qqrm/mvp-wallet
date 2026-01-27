//! Pure core logic for double-entry bookkeeping and balance projection.
//!
//! This module is intentionally DB-agnostic so it can be tested easily.

use crate::domain::{AccountId, AmountMinor, Currency, EntryDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntrySpec {
    pub account_id: AccountId,
    pub currency: Currency,
    pub direction: EntryDirection,
    pub amount_minor: AmountMinor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionDelta {
    pub account_id: AccountId,
    pub currency: Currency,
    pub available_delta_minor: i64,
    pub hold_delta_minor: i64,
}

/// Build a canonical pair of entries for a double-entry operation:
/// - `debit_account`: DEBIT `amount` (available decreases)
/// - `credit_account`: CREDIT `amount` (available increases)
pub fn build_double_entry(
    debit_account: AccountId,
    credit_account: AccountId,
    currency: &Currency,
    amount_minor: AmountMinor,
) -> [EntrySpec; 2] {
    [
        EntrySpec {
            account_id: debit_account,
            currency: currency.clone(),
            direction: EntryDirection::Debit,
            amount_minor,
        },
        EntrySpec {
            account_id: credit_account,
            currency: currency.clone(),
            direction: EntryDirection::Credit,
            amount_minor,
        },
    ]
}

/// Convert entries into balance projection deltas (available/hold).
///
/// Current semantics:
/// - DEBIT reduces available
/// - CREDIT increases available
/// - hold is unchanged
pub fn projection_deltas_from_entries(entries: &[EntrySpec]) -> Vec<ProjectionDelta> {
    // Heap-light aggregation: we avoid map overhead and keep logic purely linear.
    // Complexity is O(n^2) worst-case, but `entries` is tiny in wallet operations (double-entry => 2 entries).
    let mut deltas: Vec<ProjectionDelta> = Vec::new();

    for e in entries {
        let signed = e.direction.apply_sign(e.amount_minor.as_i64());

        if let Some(existing) = deltas
            .iter_mut()
            .find(|d| d.account_id == e.account_id && d.currency == e.currency)
        {
            existing.available_delta_minor = existing.available_delta_minor.saturating_add(signed);
            continue;
        }

        deltas.push(ProjectionDelta {
            account_id: e.account_id,
            currency: e.currency.clone(),
            available_delta_minor: signed,
            hold_delta_minor: 0,
        });
    }

    deltas
}

/// Convenience helper: the sum of signed deltas across all accounts should be zero
/// for any well-formed double-entry operation.
pub fn check_zero_sum(deltas: &[ProjectionDelta]) -> bool {
    let sum: i64 = deltas
        .iter()
        .map(|d| d.available_delta_minor)
        .fold(0_i64, |acc, x| acc.saturating_add(x));
    sum == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    #[test]
    fn topup_double_entry_has_correct_directions() {
        let currency = Currency::parse("uzs").unwrap();
        let amount = AmountMinor::new(100).unwrap();

        let system_cash = AccountId::new(1);
        let user = AccountId::new(2);

        let entries = build_double_entry(system_cash, user, &currency, amount);

        assert_eq!(entries[0].account_id, system_cash);
        assert_eq!(entries[0].direction, EntryDirection::Debit);
        assert_eq!(entries[1].account_id, user);
        assert_eq!(entries[1].direction, EntryDirection::Credit);
    }

    #[test]
    fn projection_deltas_have_zero_sum_for_double_entry() {
        let currency = Currency::parse("USD").unwrap();
        let amount = AmountMinor::new(42).unwrap();

        let entries = build_double_entry(AccountId::new(10), AccountId::new(11), &currency, amount);
        let deltas = projection_deltas_from_entries(&entries);

        assert!(check_zero_sum(&deltas));
    }

    #[test]
    fn projection_deltas_apply_expected_per_account() {
        let currency = Currency::parse("USD").unwrap();
        let amount = AmountMinor::new(500).unwrap();

        let debit = AccountId::new(100);
        let credit = AccountId::new(200);

        let entries = build_double_entry(debit, credit, &currency, amount);
        let deltas = projection_deltas_from_entries(&entries);

        let mut debit_delta = None;
        let mut credit_delta = None;
        for d in &deltas {
            if d.account_id == debit {
                debit_delta = Some(d.available_delta_minor);
            }
            if d.account_id == credit {
                credit_delta = Some(d.available_delta_minor);
            }
        }

        assert_eq!(debit_delta, Some(-amount.as_i64()));
        assert_eq!(credit_delta, Some(amount.as_i64()));
        assert!(check_zero_sum(&deltas));
    }

    #[test]
    fn deltas_aggregate_multiple_entries_for_same_account_and_currency() {
        let currency = Currency::parse("EUR").unwrap();
        let amount1 = AmountMinor::new(10).unwrap();
        let amount2 = AmountMinor::new(15).unwrap();

        let debit = AccountId::new(1);
        let credit = AccountId::new(2);

        let mut entries = Vec::new();
        entries.extend_from_slice(&build_double_entry(debit, credit, &currency, amount1));
        entries.extend_from_slice(&build_double_entry(debit, credit, &currency, amount2));

        let deltas = projection_deltas_from_entries(&entries);

        let expected_total = amount1.as_i64() + amount2.as_i64();

        let mut debit_delta = None;
        let mut credit_delta = None;
        for d in &deltas {
            if d.account_id == debit {
                debit_delta = Some(d.available_delta_minor);
            }
            if d.account_id == credit {
                credit_delta = Some(d.available_delta_minor);
            }
        }

        assert_eq!(debit_delta, Some(-expected_total));
        assert_eq!(credit_delta, Some(expected_total));
        assert!(check_zero_sum(&deltas));
    }

    proptest! {
        #[test]
        fn any_amount_produces_zero_sum_deltas(amount in 1u64..1_000_000u64) {
            let currency = Currency::parse("EUR").unwrap();
            let amount = AmountMinor::new(amount).unwrap();

            let entries = build_double_entry(AccountId::new(1), AccountId::new(2), &currency, amount);
            let deltas = projection_deltas_from_entries(&entries);

            prop_assert!(check_zero_sum(&deltas));
        }

        #[test]
        fn any_two_amounts_aggregate_correctly(a in 1u64..1_000_000u64, b in 1u64..1_000_000u64) {
            let currency = Currency::parse("UZS").unwrap();
            let a = AmountMinor::new(a).unwrap();
            let b = AmountMinor::new(b).unwrap();

            let debit = AccountId::new(1);
            let credit = AccountId::new(2);

            let mut entries = Vec::new();
            entries.extend_from_slice(&build_double_entry(debit, credit, &currency, a));
            entries.extend_from_slice(&build_double_entry(debit, credit, &currency, b));

            let deltas = projection_deltas_from_entries(&entries);

            let expected_total = a.as_i64() + b.as_i64();

            let mut debit_delta = None;
            let mut credit_delta = None;
            for d in &deltas {
                if d.account_id == debit {
                    debit_delta = Some(d.available_delta_minor);
                }
                if d.account_id == credit {
                    credit_delta = Some(d.available_delta_minor);
                }
            }

            prop_assert_eq!(debit_delta, Some(-expected_total));
            prop_assert_eq!(credit_delta, Some(expected_total));
            prop_assert!(check_zero_sum(&deltas));
        }
    }
}
