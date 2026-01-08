use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, oneshot};

/// Симуляция функции handle_incoming для бенчмарков
async fn handle_incoming_bench(
    text: String,
    pending_requests: &Arc<Mutex<HashMap<u64, oneshot::Sender<String>>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return,
    };

    // RPC response: has "id"
    if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
        let mut pending = pending_requests.lock().await;
        if let Some(tx) = pending.remove(&id) {
            let _ = tx.send(text);
        }
        return;
    }

    // Subscription notification: has "channel_name"
    if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
        let mut subs = subscriptions.lock().await;
        if let Some(sender) = subs.get_mut(channel_name) {
            let _ = sender.send(text);
        }
        return;
    }
}

fn bench_handle_incoming(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("handle_incoming");

    // Создаем тестовые данные
    let rpc_response = json!({
        "jsonrpc": "2.0",
        "id": 12345,
        "result": {"status": "ok"}
    })
    .to_string();

    let ticker_message = json!({
        "channel_name": "ticker.BTC-PERPETUAL.100ms",
        "notification": {
            "mark_price": 50000.6,
            "mark_timestamp": 1234567890.123,
            "best_bid_price": 50000.5,
            "best_ask_price": 50001.0
        }
    })
    .to_string();

    // Бенчмарк: обработка RPC ответа с пустыми структурами
    group.bench_function("rpc_response_empty_structures", |b| {
        let pending = Arc::new(Mutex::new(HashMap::<u64, oneshot::Sender<String>>::new()));
        let subs = Arc::new(Mutex::new(
            HashMap::<String, mpsc::UnboundedSender<String>>::new(),
        ));
        b.to_async(&rt)
            .iter(|| handle_incoming_bench(black_box(rpc_response.clone()), &pending, &subs));
    });

    // Бенчмарк: обработка RPC ответа с pending запросом
    group.bench_function("rpc_response_with_pending", |b| {
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let subs = Arc::new(Mutex::new(HashMap::new()));
        // Предзаполняем pending
        rt.block_on(async {
            let (tx, rx) = oneshot::channel();
            let mut guard = pending.lock().await;
            guard.insert(12345, tx);
            drop(guard);
            // Читаем ответ в фоне, чтобы не блокировать
            tokio::spawn(async move {
                let _ = rx.await;
            });
        });
        b.to_async(&rt)
            .iter(|| handle_incoming_bench(black_box(rpc_response.clone()), &pending, &subs));
    });

    // Бенчмарк: обработка ticker сообщения без подписки
    group.bench_function("ticker_no_subscription", |b| {
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let subs = Arc::new(Mutex::new(HashMap::new()));
        b.to_async(&rt)
            .iter(|| handle_incoming_bench(black_box(ticker_message.clone()), &pending, &subs));
    });

    // Бенчмарк: обработка ticker сообщения с подпиской
    group.bench_function("ticker_with_subscription", |b| {
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let subs = Arc::new(Mutex::new(HashMap::new()));
        // Предзаполняем подписку
        rt.block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let mut guard = subs.lock().await;
            guard.insert("ticker.BTC-PERPETUAL.100ms".to_string(), tx);
            drop(guard);
            // Читаем сообщения в фоне
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
        });
        b.to_async(&rt)
            .iter(|| handle_incoming_bench(black_box(ticker_message.clone()), &pending, &subs));
    });

    // Бенчмарк: обработка с множеством подписок (конкуренция за блокировку)
    for subscription_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("ticker_many_subscriptions", subscription_count),
            subscription_count,
            |b, &count| {
                let pending = Arc::new(Mutex::new(HashMap::new()));
                let subs = Arc::new(Mutex::new(HashMap::new()));
                // Предзаполняем множеством подписок
                rt.block_on(async {
                    let mut guard = subs.lock().await;
                    for i in 0..count {
                        let (tx, mut rx) = mpsc::unbounded_channel();
                        guard.insert(format!("ticker.INSTR-{}.100ms", i), tx);
                        // Читаем в фоне
                        tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
                    }
                });
                b.to_async(&rt).iter(|| {
                    handle_incoming_bench(black_box(ticker_message.clone()), &pending, &subs)
                });
            },
        );
    }

    // Бенчмарк: обработка с множеством pending запросов
    for pending_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("rpc_many_pending", pending_count),
            pending_count,
            |b, &count| {
                let pending = Arc::new(Mutex::new(HashMap::new()));
                let subs = Arc::new(Mutex::new(HashMap::new()));
                // Предзаполняем pending
                rt.block_on(async {
                    let mut guard = pending.lock().await;
                    for i in 0..count {
                        let (tx, rx) = oneshot::channel();
                        guard.insert(i, tx);
                        // Читаем в фоне
                        tokio::spawn(async move {
                            let _ = rx.await;
                        });
                    }
                });
                // Используем ID, которого нет в pending (worst case - поиск по всей map)
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": count + 1000,
                    "result": {"status": "ok"}
                })
                .to_string();
                b.to_async(&rt)
                    .iter(|| handle_incoming_bench(black_box(response.clone()), &pending, &subs));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_handle_incoming);
criterion_main!(benches);
