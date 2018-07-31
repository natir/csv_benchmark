extern crate csv_multithread;

#[macro_use]
extern crate criterion;

use std::fs;
use std::time::Duration;

use criterion::*;

use csv_multithread::*;


fn bench(c: &mut Criterion) {
    c.bench(
        "mutex",
        ParameterizedBenchmark::new(
            "buff_size",
            |b, size| { b.iter(|| mutex(*size));},
            (1..=12).map(|x| (2 as usize).pow(x))
        )
        .sample_size(10)
        .warm_up_time(Duration::new(2, 0))
        .measurement_time(Duration::new(240, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("test.csv").unwrap().len() as u32))
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
