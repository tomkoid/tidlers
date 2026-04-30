use tracing_subscriber::prelude::*;
pub fn configure_tidlers_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let filter_layer = tracing_subscriber::filter::EnvFilter::from_default_env()
        .add_directive("tidlers=debug".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
