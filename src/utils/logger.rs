use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoggerSettings {
    pub log_to_file: bool,
    pub log_filter: String,
    pub log_directory: String,
    pub log_prefix: String,
}

/// Initializes the logger, optionally with Loki support.
///
/// # Arguments
/// - `loki_url`: An optional Loki URL (e.g., `Some("http://localhost:3100")`). If `None`, Loki is disabled.
/// - `service_name`: The name of the service emitting logs.
///
/// # Example
/// ```
/// init_logger(Some("http://localhost:3100"), "service1");
///
/// init_logger(None, "service1");
/// ```
pub fn init_logger(
    loki_url: Option<&str>,
    service_name: &str,
    logger_settings: &LoggerSettings,
) -> tracing_appender::non_blocking::WorkerGuard {
    // Create an EnvFilter for log levels (e.g., info, debug)
    let env_filter = tracing_subscriber::EnvFilter::new(logger_settings.log_filter.as_str());

    let (non_blocking_writer, _guard);
    if logger_settings.log_to_file {
        let file_appender = tracing_appender::rolling::hourly(
            logger_settings.log_directory.clone(),
            logger_settings.log_prefix.clone(),
        );
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    } else {
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    }

    // Build the tracing subscriber
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::EnvFilter::new(
            logger_settings.log_filter.clone(),
        ))
        .with(tracing_subscriber::fmt::Layer::new().with_writer(non_blocking_writer));

    if let Some(url) = loki_url {
        // Create a Loki layer if a URL is provided
        let (loki_layer, task) = tracing_loki::builder()
            .label("service", service_name)
            .unwrap()
            .build_url(Url::parse(url).unwrap())
            .expect("Failed to create Loki layer");

        // Spawn the Loki task for asynchronous logging
        tokio::spawn(task);

        // Add Loki layer to the subscriber
        subscriber.with(loki_layer).init();
        tracing::info!("Logger initialized with Loki for service: {}", service_name);
    } else {
        // Initialize without Loki
        //println!("{:?}", subscriber);
        subscriber.init();
        tracing::info!("Logger initialized without Loki for service:");
    }
    _guard
}
