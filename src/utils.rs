pub fn round_to_ticks(price: f64, tick_size: f64) -> f64 {
    (price / tick_size).round() * tick_size
}
