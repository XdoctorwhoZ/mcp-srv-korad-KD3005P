/// Initialize tracing for the application.
///
/// In debug builds: sets up TRACE level logging with target display,
/// writing to ~/mcp-logs/mcp-srv-korad-KD3005P.log (truncated on each run).
/// In release builds: this is a no-op.
pub fn init_tracing() {
    #[cfg(debug_assertions)]
    init_debug_tracing();
}

#[cfg(debug_assertions)]
fn init_debug_tracing() {
    use std::fs::OpenOptions;
    use std::panic;
    use std::sync::Mutex;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    // Resolve log path: ~/mcp-logs/mcp-srv-korad-KD3005P.log
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let log_dir = std::path::Path::new(&home).join("mcp-logs");
    let log_path = log_dir.join("mcp-srv-korad-KD3005P.log");

    // Create the log directory if it does not exist
    std::fs::create_dir_all(&log_dir)
        .unwrap_or_else(|e| panic!("Failed to create log directory {:?}: {}", log_dir, e));

    // Open the log file, truncating any previous content
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&log_path)
        .unwrap_or_else(|e| panic!("Failed to open log file {:?}: {}", log_path, e));

    // Build and install the global tracing subscriber (TRACE level, targets enabled)
    tracing_subscriber::registry()
        .with(EnvFilter::new("trace"))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(Mutex::new(log_file))
                .with_target(true)
                .with_ansi(false),
        )
        .init();

    // Install a panic hook that logs the panic through tracing before propagating
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        tracing::error!(panic = %info, "panic occurred");
        default_hook(info);
    }));
}
