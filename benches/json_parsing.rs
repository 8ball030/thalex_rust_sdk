use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use serde_json::{Value, json};

/// Бенчмарк для измерения производительности JSON парсинга
/// Тестирует различные размеры сообщений и типы данных
/// Eng: Benchmark to measure JSON parsing performance
/// Tests various message sizes and data types
fn bench_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_parsing");

    // RPC ответ (маленькое сообщение)
    // Eng: RPC response (small message)
    let rpc_response = json!({
        "jsonrpc": "2.0",
        "id": 12345,
        "result": {
            "instruments": ["BTC-PERPETUAL", "ETH-PERPETUAL"]
        }
    })
    .to_string();

    // Ticker сообщение (среднее сообщение)
    // Eng: Ticker message (medium message)
    let ticker_message = json!({
        "channel_name": "ticker.BTC-PERPETUAL.100ms",
        "notification": {
            "best_bid_price": 50000.5,
            "best_bid_amount": 1.2,
            "best_ask_price": 50001.0,
            "best_ask_amount": 0.8,
            "last_price": 50000.75,
            "mark_price": 50000.6,
            "mark_timestamp": 1234567890.123,
            "iv": 0.45,
            "delta": 0.5,
            "index": 50000.0,
            "forward": 50000.5,
            "volume_24h": 1000000.0,
            "value_24h": 50000000000.0,
            "low_price_24h": 49000.0,
            "high_price_24h": 51000.0,
            "change_24h": 1000.0,
            "collar_low": 49000.0,
            "collar_high": 51000.0,
            "open_interest": 50000.0,
            "funding_rate": 0.0001,
            "funding_mark": 5.0,
            "realised_funding_24h": 120.0,
            "average_funding_rate_24h": 0.0001
        }
    })
    .to_string();

    // Большое сообщение (множество инструментов)
    // Eng: Large message (many instruments)
    let large_message = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "instruments": (0..100).map(|i| format!("INSTRUMENT-{}", i)).collect::<Vec<_>>(),
            "data": (0..50).map(|i| json!({
                "id": i,
                "name": format!("Item {}", i),
                "values": (0..20).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        }
    })
    .to_string();

    // Бенчмарк: полный парсинг RPC ответа
    // Eng: Benchmark: full parsing of RPC response
    group.bench_with_input(
        BenchmarkId::new("full_parse", "rpc_response"),
        &rpc_response,
        |b, input| {
            b.iter(|| {
                let parsed: Value = serde_json::from_str(black_box(input)).unwrap();
                black_box(parsed)
            });
        },
    );

    // Бенчмарк: полный парсинг ticker сообщения
    // Eng: Benchmark: full parsing of ticker message
    group.bench_with_input(
        BenchmarkId::new("full_parse", "ticker_message"),
        &ticker_message,
        |b, input| {
            b.iter(|| {
                let parsed: Value = serde_json::from_str(black_box(input)).unwrap();
                black_box(parsed)
            });
        },
    );

    // Бенчмарк: полный парсинг большого сообщения
    // Eng: Benchmark: full parsing of large message
    group.bench_with_input(
        BenchmarkId::new("full_parse", "large_message"),
        &large_message,
        |b, input| {
            b.iter(|| {
                let parsed: Value = serde_json::from_str(black_box(input)).unwrap();
                black_box(parsed)
            });
        },
    );

    // Бенчмарк: быстрая проверка наличия ключа "id" без полного парсинга
    // Eng: Benchmark: quick check for "id" key presence without full parsing
    group.bench_with_input(
        BenchmarkId::new("check_key", "id_in_rpc"),
        &rpc_response,
        |b, input| {
            b.iter(|| {
                let has_id = input.contains(r#""id":"#);
                black_box(has_id)
            });
        },
    );

    // Бенчмарк: быстрая проверка наличия ключа "channel_name"
    // Eng: Benchmark: quick check for "channel_name" key presence
    group.bench_with_input(
        BenchmarkId::new("check_key", "channel_name_in_ticker"),
        &ticker_message,
        |b, input| {
            b.iter(|| {
                let has_channel = input.contains(r#""channel_name":"#);
                black_box(has_channel)
            });
        },
    );

    // Бенчмарк: парсинг только после проверки ключа
    // Eng: Benchmark: parsing only after key check
    group.bench_with_input(
        BenchmarkId::new("conditional_parse", "rpc_after_check"),
        &rpc_response,
        |b, input| {
            b.iter(|| {
                if input.contains(r#""id":"#) {
                    let parsed: Value = serde_json::from_str(black_box(input)).unwrap();
                    let id = parsed.get("id").and_then(|v| v.as_u64());
                    black_box(id)
                } else {
                    black_box(None)
                }
            });
        },
    );

    group.finish();
}

criterion_group!(benches, bench_json_parsing);
criterion_main!(benches);
