use std::path::PathBuf;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Level;
use tracing_subscriber::EnvFilter;

// -------------------------------------------------------------------------------

/// Get the user root directory for Panduza
///
/// Returns the path to the `.panduza` directory inside the user's home directory.
///
/// # Returns
/// `Some(PathBuf)` containing the path to `~/.panduza`,
/// or `None` if the home directory cannot be determined.
pub fn user_root_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".panduza"))
}
pub fn mcp_logs_dir() -> Option<PathBuf> {
    user_root_dir().map(|dir| dir.join("mcp-logs"))
}

#[derive(Default, Clone, Debug)]
/// Builder for the Dioxus logger
pub struct TraceBootstrap {
    /// Minimum log level
    pub level: Option<Level>,
    /// Additional filters
    pub filters: Vec<String>,
    /// Whether to display the target of the log
    pub display_target: bool,
    /// Whether to display function span events
    pub with_fn_spans: bool,
    /// Optional file path to write logs to
    pub file_path: Option<String>,
}

impl TraceBootstrap {
    /// Create a new TraceHelper
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    pub fn display_target(mut self, display: bool) -> Self {
        self.display_target = display;
        self
    }

    /// Enable or disable function span events
    pub fn with_fn_spans(mut self, enable: bool) -> Self {
        self.with_fn_spans = enable;
        self
    }

    // /// Add a filter to the logger
    // pub fn add_filter(mut self, filter: &str) -> Self {
    //     self.filters.push(filter.into());
    //     self
    // }

    // pub fn set_filters(mut self, filters: Vec<String>) -> Self {
    //     self.filters = filters;
    //     self
    // }

    // pub fn filter_rumqttd(mut self) -> Self {
    //     self.filters.push("rumqttd=off".into());
    //     self
    // }

    // pub fn filter_dioxus_core(mut self) -> Self {
    //     self.filters.push("dioxus_core=off".into());
    //     self
    // }

    // pub fn filter_dioxus_signals(mut self) -> Self {
    //     self.filters.push("dioxus_signals=off".into());
    //     self
    // }

    // pub fn filter_warnings(mut self) -> Self {
    //     self.filters.push("warnings=off".into());
    //     self
    // }

    pub fn filter_rmcp(mut self) -> Self {
        self.filters.push("rmcp=off".into());
        self
    }

    // /// Write logs to a file instead of stdout
    // pub fn on_file(mut self, path: &str) -> Self {
    //     self.file_path = Some(path.into());
    //     self
    // }

    pub fn on_mcp_file(mut self, name: &str) -> Self {
        mcp_logs_dir().map(|dir| {
            let path = dir.join(format!("{}.log", name));
            self.file_path = Some(path.to_string_lossy().to_string());
        });
        self
    }

    pub fn build(self) -> Result<(), SetGlobalDefaultError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Create a filter that keeps the default level but excludes rumqttd logs
            let level_str = match self.level.unwrap_or(Level::INFO) {
                Level::ERROR => "error",
                Level::WARN => "warn",
                Level::INFO => "info",
                Level::DEBUG => "debug",
                Level::TRACE => "trace",
            };

            let filter_str = format!("{},{}", level_str, self.filters.join(","));
            let filter = EnvFilter::builder().parse_lossy(&filter_str);

            let mut sub = tracing_subscriber::FmtSubscriber::builder();

            if self.with_fn_spans {
                sub = sub.with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::ENTER
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                );
            }

            let sub = sub.with_env_filter(filter);

            // If file path is specified, write to file with timestamps
            if let Some(ref path) = self.file_path {
                let path = std::path::Path::new(path);
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .expect("Failed to create parent directories for log file");
                }

                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .expect("Failed to open log file");

                return set_global_default(
                    sub.with_ansi(false)
                        .with_writer(std::sync::Arc::new(file))
                        .with_target(self.display_target)
                        .finish(),
                );
            } else {
                // Otherwise, write to stdout without timestamps
                return set_global_default(
                    sub.without_time().with_target(self.display_target).finish(),
                );
            }
        }
    }
}
