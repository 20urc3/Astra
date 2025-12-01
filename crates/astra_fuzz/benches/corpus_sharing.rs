use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use crossbeam::queue::SegQueue;
use flume;
use std::sync::Arc;
use std::thread;

fn segqueue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_comparison");
    
    for num_threads in [12] {
        group.bench_with_input(
            BenchmarkId::new("SegQueue", num_threads),
            &num_threads,
            |b, &threads| {
                b.iter(|| {
                    let queue = Arc::new(SegQueue::new());
                    for i in 0..100000 {
                        queue.push(i);
                    }
                    
                    let handles: Vec<_> = (0..threads)
                        .map(|_| {
                            let q = queue.clone();
                            thread::spawn(move || {
                                for _ in 0..100 {
                                    if let Some(val) = q.pop() {
                                        black_box(val);
                                        q.push(val + 1);
                                    }
                                }
                            })
                        })
                        .collect();
                    
                    for h in handles {
                        h.join().unwrap();
                    }
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("Flume", num_threads),
            &num_threads,
            |b, &threads| {
                b.iter(|| {
                    let (tx, rx) = flume::unbounded();
                    for i in 0..100000 {
                        tx.send(i).unwrap();
                    }
                    
                    let handles: Vec<_> = (0..threads)
                        .map(|_| {
                            let tx_clone = tx.clone();
                            let rx_clone = rx.clone();
                            thread::spawn(move || {
                                for _ in 0..100 {
                                    if let Ok(val) = rx_clone.try_recv() {
                                        black_box(val);
                                        tx_clone.send(val + 1).unwrap();
                                    }
                                }
                            })
                        })
                        .collect();
                    
                    for h in handles {
                        h.join().unwrap();
                    }
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, segqueue_benchmark);
criterion_main!(benches);