use rust_decimal::Decimal;

pub fn round_to_ticks(price: Decimal, tick_size: Decimal) -> Decimal {
    (price / tick_size).round() * tick_size
}
