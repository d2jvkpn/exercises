use tracing::subscriber::set_global_default; // Subscriber
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_subscriber(name: String, env_filter: String) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber =
        Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    // impl Subscriber + Send + Sync

    // setup tracing
    // Redirect all `log`'s events to our subscriber
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
