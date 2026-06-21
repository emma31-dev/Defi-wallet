use tracing_appender::{rolling::daily, non_blocking};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing::Level;

pub fn init_logging() {
    let file_appender = daily("/../logs", "app.log");

    let (non_blocking, _guard) = non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_line_number(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_line_number(true);

    // 6. Set up the subscriber with environment filter
    let env_filter = EnvFilter::from_default_env().add_directive(Level::DEBUG.into());

    // 7. Register the subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();
}
