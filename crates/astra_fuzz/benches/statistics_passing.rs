use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use criterion::{criterion_group, criterion_main, Criterion};

const NUM_THREADS: usize = 32;
const OPS_PER_THREAD: usize = 100;

fn bench_mutex(c: &mut Criterion) {
    c.bench_function("mutex_stats_passing", |b| {
        b.iter(|| {
            let counter = Arc::new(Mutex::new(0u32));
            
            let handles: Vec<_> = (0..NUM_THREADS).map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..OPS_PER_THREAD {
                        let mut n = counter.lock().unwrap();
                        *n += 1;
                    }
                })
            }).collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn bench_atomic(c: &mut Criterion) {
    c.bench_function("atomic_stats_passing", |b| {
        b.iter(|| {
            let counter = Arc::new(AtomicU32::new(0));
            
            let handles: Vec<_> = (0..NUM_THREADS).map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..OPS_PER_THREAD {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                })
            }).collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn bench_channel(c: &mut Criterion) {
    c.bench_function("channel_stats_passing", |b| {
        b.iter(|| {
            let (tx, rx) = channel::<u32>();
            
            let handles: Vec<_> = (0..NUM_THREADS).map(|_| {
                let tx = tx.clone();
                thread::spawn(move || {
                    for _ in 0..OPS_PER_THREAD {
                        tx.send(1).unwrap();
                    }
                })
            }).collect();
            
            let receiver = thread::spawn(move || {
                for _ in 0..(NUM_THREADS * OPS_PER_THREAD) {
                    let _ = rx.recv().unwrap();
                }
            });
            
            for handle in handles {
                handle.join().unwrap();
            }
            receiver.join().unwrap();
        })
    });
}

criterion_group!(benches, bench_mutex, bench_atomic, bench_channel);
criterion_main!(benches);