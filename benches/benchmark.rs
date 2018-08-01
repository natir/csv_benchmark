extern crate csv_multithread;

#[macro_use]
extern crate criterion;

use std::fs;
use std::time::Duration;

use criterion::*;

use csv_multithread::*;

fn buff_size(c: &mut Criterion) {
    c.bench(
        "mutex",
        ParameterizedBenchmark::new(
            "buffsize",
            |b, size| { b.iter(|| mutex(*size));},
            (1..=12).map(|x| (2 as usize).pow(x))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        //.measurement_time(Duration::new(240, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("test.csv").unwrap().len() as u32))
    );
    
    c.bench(
        "messsage",
        ParameterizedBenchmark::new(
            "buffsize",
            |b, size| { b.iter(|| mutex(*size));},
            (1..=12).map(|x| (2 as usize).pow(x))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        //.measurement_time(Duration::new(240, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("test.csv").unwrap().len() as u32))
    );
}

fn compare(c: &mut Criterion) {
    c.bench(
        "compare",
        Benchmark::new("mutex", |b| { b.iter(|| mutex(128));})
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        //.measurement_time(Duration::new(240, 0))
        .throughput(Throughput::Bytes(fs::metadata("test.csv").unwrap().len() as u32))
        .with_function("basic", |b| { b.iter(|| basic())})
        .with_function("message", |b| {b.iter(|| message(128))})
    );
}

criterion_group!(benches, buff_size, compare);
criterion_main!(benches);
