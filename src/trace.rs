use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

// -------------------------------------------------------------------------------
// TraceBootstrap
// -------------------------------------------------------------------------------

#[derive(Default, Clone, Debug)]
/// Builder for the tracing subscriber.
pub struct TraceBootstrap {
    /// Minimum log level
    pub level: Option<Level>,
    /// Additional filters
    pub filters: Vec<String>,
    /// Whether to display the target of the log
    pub display_target: bool,
    /// Whether to display function span events
    pub with_fn_spans: bool,
}

impl TraceBootstrap {
    /// Set the minimum log level.
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    /// Enable or disable display of the log target.
    pub fn display_target(mut self, display: bool) -> Self {
        self.display_target = display;
        self
    }

    /// Enable or disable function span events.
    pub fn with_fn_spans(mut self, enable: bool) -> Self {
        self.with_fn_spans = enable;
        self
    }

    /// Filter out rmcp logs.
    pub fn filter_rmcp(mut self) -> Self {
        self.filters.push("rmcp=off".into());
        self
    }

    /// Build and install the global tracing subscriber.
    pub fn build(self) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
        let level_str = match self.level.unwrap_or(Level::INFO) {
            Level::ERROR => "error",
            Level::WARN => "warn",
            Level::INFO => "info",
            Level::DEBUG => "debug",
            Level::TRACE => "trace",
        };

        let filter_str = format!("{},{}", level_str, self.filters.join(","));
        let filter = EnvFilter::builder().parse_lossy(&filter_str);

        let mut fmt_layer_builder = tracing_subscriber::fmt::layer()
            .without_time()
            .with_target(self.display_target);

        if self.with_fn_spans {
            fmt_layer_builder = fmt_layer_builder.with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            );
        }

        let registry = tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer_builder);

        set_global_default(registry)
    }
}
