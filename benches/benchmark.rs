extern crate csv_multithread;

#[macro_use]
extern crate criterion;

extern crate itertools;

use std::fs;
use std::time::Duration;

use itertools::Itertools;

use criterion::*;

use csv_multithread::*;

fn file_size(c: &mut Criterion) {
    c.bench(
        "mutex",
        ParameterizedBenchmark::new(
            "filesize",
            |b, file| { b.iter(|| mutex(format!("{}.paf", file).as_str(), 256, 4));},
            (1..=6).map(|x| x*2)
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|file| Throughput::Bytes(fs::metadata(format!("{}.paf", file)).unwrap().len() as u32))
    );

    c.bench(
        "messsage",
        ParameterizedBenchmark::new(
            "filesize",
            |b, file| { b.iter(|| mutex(format!("{}.paf", file).as_str(), 256, 4));},
            (1..=6).map(|x| x*2)
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|file| Throughput::Bytes(fs::metadata(format!("{}.paf", file)).unwrap().len() as u32))
    );
}

fn buff_size(c: &mut Criterion) {
    c.bench(
        "mutex",
        ParameterizedBenchmark::new(
            "buffsize",
            |b, size| { b.iter(|| mutex("8.paf", *size, 4));},
            (1..=12).map(|x| (2 as usize).pow(x))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
    
    c.bench(
        "messsage",
        ParameterizedBenchmark::new(
            "buffsize",
            |b, size| { b.iter(|| message("8.paf", *size, 4));},
            (1..=12).map(|x| (2 as usize).pow(x))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
}

fn nb_thread(c: &mut Criterion) {
    c.bench(
        "mutex",
        ParameterizedBenchmark::new(
            "nbthread",
            |b, thread| { b.iter(|| mutex("8.paf", 256, *thread));},
            (1..=12).map(|x| x*2)
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
    
    c.bench(
        "messsage",
        ParameterizedBenchmark::new(
            "nbthread",
            |b, thread| { b.iter(|| message("8.paf", 256, *thread));},
            (1..=12).map(|x| x*2)
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
}

fn buffsize_nb_thread(c: &mut Criterion) {
    c.bench(
        "buffsize-nbthread",
        ParameterizedBenchmark::new(
            "mutex",
            |b, param| { b.iter(|| message("8.paf", param.0, param.1));},
            (1..=12).map(|x| (2 as usize).pow(x)).cartesian_product((1..=12).map(|x| x*2))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
    
    c.bench(
        "buffsize-nbthread",
        ParameterizedBenchmark::new(
            "messsage",
            |b, param| { b.iter(|| message("8.paf", param.0, param.1));},
            (1..=12).map(|x| (2 as usize).pow(x)).cartesian_product((1..=12).map(|x| x*2))
        )
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(|_| Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
    );
}

fn compare(c: &mut Criterion) {
    c.bench(
        "compare",
        Benchmark::new("mutex", |b| { b.iter(|| mutex("8.paf", 256, 4));})
        .sample_size(40)
        .warm_up_time(Duration::new(2, 0))
        .throughput(Throughput::Bytes(fs::metadata("8.paf").unwrap().len() as u32))
        .with_function("basic", |b| { b.iter(|| basic("8.paf"))})
        .with_function("message", |b| {b.iter(|| message("8.paf", 256, 4))})
    );
}

criterion_group!(benches, file_size, buff_size, nb_thread, buffsize_nb_thread, compare);
criterion_main!(benches);
