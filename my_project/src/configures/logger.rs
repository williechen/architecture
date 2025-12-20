use chrono::Local;
use serde::{Deserialize, Serialize};
use tracing::Subscriber;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::Layer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::prelude::*;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerConfig {
    pub level: Option<String>,
    pub log_directory: Option<String>,
    pub file_prefix: Option<String>,
}

impl LoggerConfig {
    pub fn load(&self) -> Vec<WorkerGuard> {
        let mut guards = Vec::new();

        let level = self.level.as_deref().unwrap_or("info");
        let log_directory = self.log_directory.as_deref().unwrap_or("/tmp");
        let file_prefix = self.file_prefix.as_deref().unwrap_or("app");

        // All Levels
        let (nb_all, g_all) = Self::make_appender(log_directory, file_prefix, "all");
        guards.push(g_all);

        // Trace
        let (nb_trace, g_trace) = Self::make_appender(log_directory, file_prefix, "trace");
        guards.push(g_trace);

        // Debug
        let (nb_debug, g_debug) = Self::make_appender(log_directory, file_prefix, "debug");
        guards.push(g_debug);

        // Info
        let (nb_info, g_info) = Self::make_appender(log_directory, file_prefix, "info");
        guards.push(g_info);

        // Warn
        let (nb_warn, g_warn) = Self::make_appender(log_directory, file_prefix, "warn");
        guards.push(g_warn);

        // Error
        let (nb_error, g_error) = Self::make_appender(log_directory, file_prefix, "error");
        guards.push(g_error);

        // golbal level
        let level_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

        // 集合
        let subscriber = tracing_subscriber::registry()
            .with(level_filter)
            .with(Self::console_layer(LevelFilter::TRACE))
            .with(Self::file_level(nb_all, LevelFilter::TRACE))
            .with(Self::file_level_only(nb_trace, LevelFilter::TRACE))
            .with(Self::file_level_only(nb_debug, LevelFilter::DEBUG))
            .with(Self::file_level_only(nb_info, LevelFilter::INFO))
            .with(Self::file_level_only(nb_warn, LevelFilter::WARN))
            .with(Self::file_level_only(nb_error, LevelFilter::ERROR));

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        guards
    }

    fn make_appender(dir: &str, prefix: &str, suffix: &str) -> (NonBlocking, WorkerGuard) {
        let file = tracing_appender::rolling::hourly(dir, format!("{}_{}.log", prefix, suffix));
        tracing_appender::non_blocking(file)
    }

    /// Console layer（可指定 Level）
    fn console_layer<S>(level: LevelFilter) -> impl Layer<S> + Send + Sync
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_thread_ids(true)
            .with_timer(LocalTimer)
            .with_filter(level)
    }

    fn file_level<S>(block: NonBlocking, level: LevelFilter) -> impl Layer<S> + Send + Sync
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_thread_ids(true)
            .with_writer(block)
            .with_timer(LocalTimer)
            .with_filter(level)
    }

    fn file_level_only<S>(block: NonBlocking, level: LevelFilter) -> impl Layer<S> + Send + Sync
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        let level_only =
            tracing_subscriber::filter::filter_fn(move |metadata| metadata.level() == &level);

        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_thread_ids(true)
            .with_writer(block)
            .with_timer(LocalTimer)
            .with_filter(level_only)
    }
}
