use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static TRACER_PROVIDER: std::sync::OnceLock<SdkTracerProvider> = std::sync::OnceLock::new();
static LOGGER_PROVIDER: std::sync::OnceLock<SdkLoggerProvider> = std::sync::OnceLock::new();
static METER_PROVIDER: std::sync::OnceLock<SdkMeterProvider> = std::sync::OnceLock::new();

/// Initialize telemetry: stdout logging + optional OTLP export.
///
/// OTLP export is enabled only when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
/// Without it, only the `fmt` (stdout) layer is active.
pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer();

    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();

    if let Some(endpoint) = otel_endpoint {
        let otlp_trace_exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(&endpoint)
            .build()
            .expect("failed to create OTLP trace exporter");

        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(otlp_trace_exporter)
            .build();

        let tracer = tracer_provider.tracer("arcadia");
        TRACER_PROVIDER
            .set(tracer_provider)
            .expect("tracer provider already set");

        // Metrics exporter
        let otlp_metrics_exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint(&endpoint)
            .build()
            .expect("failed to create OTLP metrics exporter");

        let meter_provider = SdkMeterProvider::builder()
            .with_periodic_exporter(otlp_metrics_exporter)
            .build();

        METER_PROVIDER
            .set(meter_provider.clone())
            .expect("meter provider already set");

        opentelemetry::global::set_meter_provider(meter_provider.clone());

        let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let otel_metrics_layer = tracing_opentelemetry::MetricsLayer::new(meter_provider);

        let otlp_log_exporter = opentelemetry_otlp::LogExporter::builder()
            .with_tonic()
            .with_endpoint(&endpoint)
            .build()
            .expect("failed to create OTLP log exporter");

        let logger_provider = SdkLoggerProvider::builder()
            .with_batch_exporter(otlp_log_exporter)
            .build();

        let otel_log_layer = opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(
            &logger_provider,
        );

        LOGGER_PROVIDER
            .set(logger_provider)
            .expect("logger provider already set");

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(otel_trace_layer)
            .with(otel_metrics_layer)
            .with(otel_log_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

/// Flush and shut down OpenTelemetry providers gracefully.
pub fn shutdown_telemetry() {
    if let Some(tracer_provider) = TRACER_PROVIDER.get()
        && let Err(err) = tracer_provider.shutdown()
    {
        eprintln!("failed to shutdown tracer provider: {err}");
    }
    if let Some(logger_provider) = LOGGER_PROVIDER.get()
        && let Err(err) = logger_provider.shutdown()
    {
        eprintln!("failed to shutdown logger provider: {err}");
    }
    if let Some(meter_provider) = METER_PROVIDER.get()
        && let Err(err) = meter_provider.shutdown()
    {
        eprintln!("failed to shutdown meter provider: {err}");
    }
}
