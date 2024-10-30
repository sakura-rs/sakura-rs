pub fn init(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .init();
}
