use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use nostr_sdk::prelude::*;
use tokio::time::timeout;

const RELAYS: [&str; 4] = [
    "wss://no.str.cr",
    "wss://nostr.bitcoiner.social",
    "wss://relay.snort.social",
    "wss://relay.damus.io",
];

#[tokio::main]
async fn main() {
    env_logger::init();
    let kinds = Arc::new(Mutex::new(BTreeMap::new()));

    let keys = Keys::generate();
    let client = Client::new(&keys);

    for url in RELAYS {
        client
            .add_relay(url, None)
            .await
            .unwrap_or_else(|_| panic!("{} connects", url));
    }
    client.connect().await;

    let filter = Filter::new().since(Timestamp::now() - Duration::from_secs(60 * 60 * 24 * 90));
    client.subscribe(vec![filter]).await;

    let mut notifications = client.notifications();

    let mut args = std::env::args();
    let secs = args.nth(1).unwrap_or("60".to_string()).parse().unwrap();
    let _ = timeout(Duration::from_secs(secs), async {
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event(_, event) = notification {
                kinds
                    .lock()
                    .unwrap()
                    .entry(event.kind.as_u64())
                    .or_insert(AtomicU64::default())
                    .fetch_add(1, Ordering::Relaxed);
            } else {
                log::trace!("skipping notification: {:?}", notification);
            }
        }
    })
    .await;

    for (kind, n) in kinds.lock().unwrap().iter() {
        println!("{}: {}", kind, n.load(Ordering::Relaxed));
    }
}
