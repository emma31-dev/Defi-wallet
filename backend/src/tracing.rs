use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, filter::LevelFilter};

pub fn init_logging() -> WorkerGuard {
    let file_appender = daily("../logs", "app.log");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

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

    let env_filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    guard
}
