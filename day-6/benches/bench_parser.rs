use criterion::{criterion_group, criterion_main, Criterion, Fun};

use day_6_parser::{
    default_parse::parse_point as default_parse, nom_parse::parse_point as nom_parse,
};

fn parsers_valid(c: &mut Criterion) {
    let nom_parser = Fun::new("nom", |b, i| b.iter(|| nom_parse(*i)));
    let split_parser = Fun::new("split", |b, i| b.iter(|| default_parse(*i)));

    let funcs = vec![nom_parser, split_parser];

    c.bench_functions("Parsers (valid)", funcs, "123123213, 3212132312");
}

fn parsers_invalid(c: &mut Criterion) {
    let nom_parser = Fun::new("nom", |b, i| b.iter(|| nom_parse(*i)));
    let split_parser = Fun::new("split", |b, i| b.iter(|| default_parse(*i)));

    let funcs = vec![nom_parser, split_parser];

    c.bench_functions("Parsers (invalid)", funcs, "123434254, a5adfsaffdsfda321");
}

criterion_group!(benches, parsers_valid, parsers_invalid);
criterion_main!(benches);
