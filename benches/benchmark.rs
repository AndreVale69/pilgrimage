use criterion::{Criterion, criterion_group, criterion_main};
use pilgrimage::broker::{Broker, Node};
use std::sync::{Arc, Mutex};

fn broker_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("broker");

    group.bench_function("add_node", |b| {
        b.iter(|| {
            let broker = Broker::new("broker1", 3, 2, "logs");
            let node = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            broker.add_node("node1".to_string(), node);
        })
    });

    group.bench_function("replicate_data", |b| {
        b.iter(|| {
            let broker = Broker::new("broker1", 3, 2, "logs");
            let node = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            broker.add_node("node1".to_string(), node);

            let data = b"test data";
            broker.replicate_data(1, data);
        })
    });

    group.bench_function("start_election", |b| {
        b.iter(|| {
            let broker = Broker::new("broker1", 3, 2, "logs");
            let node1 = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            let node2 = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            broker.add_node("node1".to_string(), node1);
            broker.add_node("node2".to_string(), node2);

            broker.start_election();
        })
    });

    group.bench_function("failover", |b| {
        b.iter(|| {
            let broker = Broker::new("broker1", 3, 2, "logs");
            let node1 = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            let node2 = Node {
                data: Arc::new(Mutex::new(Vec::new())),
            };
            broker.add_node("node1".to_string(), node1);
            broker.add_node("node2".to_string(), node2);

            broker.detect_failure("node1");
        })
    });

    group.finish();
}

criterion_group!(benches, broker_benchmark);
criterion_main!(benches);
