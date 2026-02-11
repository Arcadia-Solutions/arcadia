use actix_web::web::Data;
use opentelemetry::metrics::Counter;

use crate::Tracker;

#[derive(Debug)]
pub struct Instruments {
    pub announces_ok: Counter<u64>,
    pub announces_err: Counter<u64>,
}

pub fn register(tracker: &Data<Tracker>, service_name: &str) {
    let scope = opentelemetry::InstrumentationScope::builder(service_name.to_string()).build();
    let meter = opentelemetry::global::meter_with_scope(scope);

    let t = tracker.clone();
    let _torrents = meter
        .u64_observable_gauge("torrents")
        .with_description("Total number of torrents")
        .with_callback(move |observer| {
            let torrents = t.torrents.lock();
            observer.observe(torrents.len() as u64, &[]);
        })
        .build();

    let t = tracker.clone();
    let _seeders = meter
        .u64_observable_gauge("seeders")
        .with_description("Total number of seeders across all torrents")
        .with_callback(move |observer| {
            let torrents = t.torrents.lock();
            let total = torrents.values().map(|t| t.seeders as u64).sum();
            observer.observe(total, &[]);
        })
        .build();

    let t = tracker.clone();
    let _leechers = meter
        .u64_observable_gauge("leechers")
        .with_description("Total number of leechers across all torrents")
        .with_callback(move |observer| {
            let torrents = t.torrents.lock();
            let total = torrents.values().map(|t| t.leechers as u64).sum();
            observer.observe(total, &[]);
        })
        .build();

    let t = tracker.clone();
    let _snatches = meter
        .u64_observable_gauge("snatches")
        .with_description("Total number of snatches across all torrents")
        .with_callback(move |observer| {
            let torrents = t.torrents.lock();
            let total = torrents.values().map(|t| t.times_completed as u64).sum();
            observer.observe(total, &[]);
        })
        .build();

    let instruments = Instruments {
        announces_ok: meter
            .u64_counter("announces.ok")
            .with_description("Total number of successful announces")
            .build(),
        announces_err: meter
            .u64_counter("announces.err")
            .with_description("Total number of errored announces")
            .build(),
    };

    let _ = tracker.metrics.set(instruments);
}
