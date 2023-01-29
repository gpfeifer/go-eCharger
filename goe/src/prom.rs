/*
pub static ref INCOMING_REQUESTS: IntCounter =
IntCounter::new("incoming_requests", "Incoming Requests").expect("metric can be created");
*/

use once_cell::sync::Lazy;
use prometheus::{Registry, IntGauge};

use crate::goe::Status;

static ENERGY: Lazy<IntGauge> = Lazy::new(|| {
    IntGauge::new("goe_energy", "Total enery").expect("metric can be created")
});

static REGISTRY: Lazy<Registry> = Lazy::new(|| {
    let registry = Registry::new();

    registry
        .register(Box::new(ENERGY.clone()))
        .expect("collector can be registered");

    registry
});

pub(crate) fn metrics(status: Status) -> String {
    use prometheus::Encoder;

    ENERGY.set(status.energy());

    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();
    res
    
}