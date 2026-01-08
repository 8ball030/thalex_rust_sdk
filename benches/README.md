# Бенчмарки производительности

Этот каталог содержит бенчмарки для измерения производительности критических компонентов Thalex Rust SDK.

## Быстрый старт

```bash
# Запустить все бенчмарки
cargo bench

# Запустить конкретный бенчмарк
cargo bench --bench json_parsing
cargo bench --bench handle_incoming
cargo bench --bench mutex_contention
cargo bench --bench subscription_throughput
```

## Доступные бенчмарки

1. **json_parsing** - производительность JSON парсинга
2. **handle_incoming** - обработка входящих сообщений
3. **mutex_contention** - производительность блокировок
4. **subscription_throughput** - пропускная способность подписок

## Результаты

Результаты бенчмарков сохраняются в `target/criterion/` с HTML отчетами для визуализации.

Подробное руководство см. в [audit/benchmark_guide.md](../audit/benchmark_guide.md).

