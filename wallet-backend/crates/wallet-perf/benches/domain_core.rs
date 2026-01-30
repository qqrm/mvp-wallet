use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wallet_domain::{
    build_double_entry, check_zero_sum, projection_deltas_from_entries, AccountId, AmountMinor,
    Currency,
};

fn bench_double_entry_and_projection(c: &mut Criterion) {
    let currency = Currency::parse("USD").unwrap();
    let amount = AmountMinor::new(1_000).unwrap();
    let debit = AccountId::new(1);
    let credit = AccountId::new(2);

    c.bench_function("domain_core/double_entry+projection", |b| {
        b.iter(|| {
            let entries = build_double_entry(
                black_box(debit),
                black_box(credit),
                black_box(&currency),
                black_box(amount),
            );
            let deltas = projection_deltas_from_entries(black_box(&entries));
            black_box(check_zero_sum(black_box(&deltas)));
        })
    });
}

criterion_group!(benches, bench_double_entry_and_projection);
criterion_main!(benches);
