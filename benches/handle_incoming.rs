use std::{hint::black_box, sync::Arc};

use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;

use thalex_rust_sdk::{types::{ChannelSender, ResponseSender}, ws_client::handle_incoming};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_tungstenite::tungstenite::Bytes;

fn bench_handle_incoming(c: &mut Criterion) {
    // ---- Shared state (NOT measured) ----
    let pending_requests: Arc<DashMap<u64, ResponseSender>> = Arc::new(DashMap::new());

    let public_subscriptions: Arc<DashMap<String, ChannelSender>> =
        Arc::new(DashMap::new());
    let private_subscriptions: Arc<DashMap<String, ChannelSender>> =
        Arc::new(DashMap::new());

    // Sample RPC response message
    let rpc_response: Bytes = r#"{"id":42,"jsonrpc":"2.0","result":"ok"}"#.into();

    // Sample subscription message
    let sub_message: Bytes = r#"{"channel_name":"ticker.BTCUSD","data":{"price":42000}}"#.into();
    let (tx, _rx) = mpsc::unbounded_channel::<Bytes>();
    public_subscriptions.insert(
        "ticker.BTCUSD".to_string(),
        tx,
    );


    c.bench_function("handle_incoming_rpc_response", |b| {
        b.iter(|| {
            // we need to add a pending request to match the id in rpc_response
            pending_requests.insert(42, tokio::sync::oneshot::channel().0);
            handle_incoming(
                black_box(rpc_response.clone()),
                black_box(&pending_requests),
                black_box(&public_subscriptions),
                black_box(&private_subscriptions),
            )
        })
    });

    c.bench_function("handle_incoming_subscription", |b| {
        b.iter(|| {
            handle_incoming(
                black_box(sub_message.clone()),
                black_box(&pending_requests),
                black_box(&public_subscriptions),
                black_box(&private_subscriptions),
            )
        })
    });
}

criterion_group!(benches, bench_handle_incoming);
criterion_main!(benches);
