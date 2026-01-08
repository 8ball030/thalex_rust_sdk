use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::oneshot;

/// Бенчмарк для измерения производительности блокировок Mutex
/// Тестирует contention при разном количестве конкурентных операций
async fn mutex_insert_remove(
    map: Arc<Mutex<HashMap<u64, oneshot::Sender<String>>>>,
    iterations: u64,
) {
    for i in 0..iterations {
        let (tx, _rx) = oneshot::channel();
        {
            let mut guard = map.lock().await;
            guard.insert(i, tx);
        }
        {
            let mut guard = map.lock().await;
            guard.remove(&i);
        }
    }
}

async fn mutex_read_heavy(map: Arc<Mutex<HashMap<u64, String>>>, keys: Vec<u64>, iterations: u64) {
    for _ in 0..iterations {
        for &key in &keys {
            let guard = map.lock().await;
            let _ = guard.get(&key);
        }
    }
}

async fn mutex_write_heavy(map: Arc<Mutex<HashMap<u64, String>>>, iterations: u64) {
    for i in 0..iterations {
        let mut guard = map.lock().await;
        guard.insert(i, format!("value_{}", i));
        guard.remove(&(i - 1));
    }
}

fn bench_mutex_contention(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("mutex_contention");

    // Бенчмарк: вставка и удаление (write-heavy)
    group.bench_function("insert_remove_100", |b| {
        let map = Arc::new(Mutex::new(HashMap::<u64, oneshot::Sender<String>>::new()));
        b.to_async(&rt)
            .iter(|| mutex_insert_remove(map.clone(), 100));
    });

    group.bench_function("insert_remove_1000", |b| {
        let map = Arc::new(Mutex::new(HashMap::<u64, oneshot::Sender<String>>::new()));
        b.to_async(&rt)
            .iter(|| mutex_insert_remove(map.clone(), 1000));
    });

    // Бенчмарк: read-heavy workload
    group.bench_function("read_heavy_10_keys", |b| {
        let map = Arc::new(Mutex::new(HashMap::new()));
        let keys: Vec<u64> = (0..10).collect();
        // Предзаполняем map
        rt.block_on(async {
            let mut guard = map.lock().await;
            for &key in &keys {
                guard.insert(key, format!("value_{}", key));
            }
        });
        b.to_async(&rt)
            .iter(|| mutex_read_heavy(map.clone(), keys.clone(), 100));
    });

    group.bench_function("read_heavy_100_keys", |b| {
        let map = Arc::new(Mutex::new(HashMap::new()));
        let keys: Vec<u64> = (0..100).collect();
        rt.block_on(async {
            let mut guard = map.lock().await;
            for &key in &keys {
                guard.insert(key, format!("value_{}", key));
            }
        });
        b.to_async(&rt)
            .iter(|| mutex_read_heavy(map.clone(), keys.clone(), 10));
    });

    // Бенчмарк: write-heavy workload
    group.bench_function("write_heavy_100", |b| {
        let map = Arc::new(Mutex::new(HashMap::<u64, String>::new()));
        b.to_async(&rt).iter(|| mutex_write_heavy(map.clone(), 100));
    });

    group.bench_function("write_heavy_1000", |b| {
        let map = Arc::new(Mutex::new(HashMap::<u64, String>::new()));
        b.to_async(&rt)
            .iter(|| mutex_write_heavy(map.clone(), 1000));
    });

    // Бенчмарк: конкурентный доступ (симуляция реального сценария)
    group.bench_function("concurrent_access_4_tasks", |b| {
        let map = Arc::new(Mutex::new(HashMap::<u64, String>::new()));
        b.to_async(&rt).iter(|| async {
            let mut handles = Vec::new();
            for task_id in 0..4 {
                let map_clone = map.clone();
                let handle = tokio::spawn(async move {
                    for i in 0..25 {
                        let key = task_id * 100 + i;
                        {
                            let mut guard = map_clone.lock().await;
                            guard.insert(key, format!("value_{}", key));
                        }
                        {
                            let guard = map_clone.lock().await;
                            let _ = guard.get(&key);
                        }
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.await.unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_mutex_contention);
criterion_main!(benches);
