use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::time::Instant;

/// Бенчмарк для измерения throughput подписок
/// Тестирует производительность при высокой частоте сообщений
async fn process_subscription_messages(
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    channel_name: &str,
    message: String,
    count: u64,
) -> u64 {
    let mut processed = 0;
    for _ in 0..count {
        let mut subs = subscriptions.lock().await;
        if let Some(sender) = subs.get_mut(channel_name) {
            if sender.send(message.clone()).is_ok() {
                processed += 1;
            }
        }
    }
    processed
}

async fn process_subscription_messages_concurrent(
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    channel_names: Vec<String>,
    message: String,
    messages_per_channel: u64,
) -> u64 {
    let mut handles = Vec::new();
    for channel_name in channel_names {
        let subs = subscriptions.clone();
        let msg = message.clone();
        let handle = tokio::spawn(async move {
            let mut processed = 0;
            for _ in 0..messages_per_channel {
                let mut guard = subs.lock().await;
                if let Some(sender) = guard.get_mut(&channel_name) {
                    if sender.send(msg.clone()).is_ok() {
                        processed += 1;
                    }
                }
            }
            processed
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }
    total
}

fn bench_subscription_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("subscription_throughput");

    let test_message =
        r#"{"channel_name":"ticker.BTC-PERPETUAL.100ms","notification":{"mark_price":50000.6}}"#
            .to_string();

    // Бенчмарк: обработка одного сообщения
    group.bench_function("single_message", |b| {
        let subs = Arc::new(Mutex::new(HashMap::new()));
        rt.block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            subs.lock()
                .await
                .insert("ticker.BTC-PERPETUAL.100ms".to_string(), tx);
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
        });
        b.to_async(&rt).iter(|| {
            process_subscription_messages(
                subs.clone(),
                "ticker.BTC-PERPETUAL.100ms",
                black_box(test_message.clone()),
                1,
            )
        });
    });

    // Бенчмарк: обработка 100 сообщений подряд
    group.bench_function("100_messages_sequential", |b| {
        let subs = Arc::new(Mutex::new(HashMap::new()));
        rt.block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            subs.lock()
                .await
                .insert("ticker.BTC-PERPETUAL.100ms".to_string(), tx);
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
        });
        b.to_async(&rt).iter(|| {
            process_subscription_messages(
                subs.clone(),
                "ticker.BTC-PERPETUAL.100ms",
                black_box(test_message.clone()),
                100,
            )
        });
    });

    // Бенчмарк: обработка 1000 сообщений подряд
    group.bench_function("1000_messages_sequential", |b| {
        let subs = Arc::new(Mutex::new(HashMap::new()));
        rt.block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            subs.lock()
                .await
                .insert("ticker.BTC-PERPETUAL.100ms".to_string(), tx);
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
        });
        b.to_async(&rt).iter(|| {
            process_subscription_messages(
                subs.clone(),
                "ticker.BTC-PERPETUAL.100ms",
                black_box(test_message.clone()),
                1000,
            )
        });
    });

    // Бенчмарк: конкурентная обработка с разным количеством каналов
    for channel_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_channels", channel_count),
            channel_count,
            |b, &count| {
                let subs = Arc::new(Mutex::new(HashMap::new()));
                let channel_names: Vec<String> = (0..count)
                    .map(|i| format!("ticker.INSTR-{}.100ms", i))
                    .collect();

                rt.block_on(async {
                    let mut guard = subs.lock().await;
                    for name in &channel_names {
                        let (tx, mut rx) = mpsc::unbounded_channel();
                        guard.insert(name.clone(), tx);
                        tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
                    }
                });

                b.to_async(&rt).iter(|| {
                    process_subscription_messages_concurrent(
                        subs.clone(),
                        channel_names.clone(),
                        black_box(test_message.clone()),
                        100,
                    )
                });
            },
        );
    }

    // Бенчмарк: измерение реального throughput (сообщений в секунду)
    group.bench_function("throughput_measurement", |b| {
        let subs = Arc::new(Mutex::new(HashMap::new()));
        let msg = test_message.clone();
        rt.block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            subs.lock()
                .await
                .insert("ticker.BTC-PERPETUAL.100ms".to_string(), tx);
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
        });
        b.to_async(&rt).iter_custom(|iters| {
            let subs_clone = subs.clone();
            let msg_clone = msg.clone();
            async move {
                let start = Instant::now();
                for _ in 0..iters {
                    let mut guard = subs_clone.lock().await;
                    if let Some(sender) = guard.get_mut("ticker.BTC-PERPETUAL.100ms") {
                        let _ = sender.send(msg_clone.clone());
                    }
                }
                start.elapsed()
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_subscription_throughput);
criterion_main!(benches);
