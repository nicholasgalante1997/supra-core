use once_cell::sync::Lazy;
use std::env;
use std::sync::Mutex;

mod debugger;
mod utils;

pub use debugger::RsDebugger;

// Global mutable default logger
static DEFAULT_LOGGER: Lazy<Mutex<RsDebugger>> = Lazy::new(|| {
    let label = env::var("DEBUG_DEFAULT_LABEL").unwrap_or("debugrs:default".to_string());
    Mutex::new(RsDebugger::new(label))
});


/// Writes a log message to the default logger.
///
/// The default logger is a global mutable logger, so be careful when using it in a
/// multithreaded environment. If you need to write logs from multiple threads, consider
/// using a thread-local logger instead.
/// 
/// To set the default logger label, set the `DEBUG_DEFAULT_LABEL` environment variable.
///
pub fn log(message: &str) {
    if let Ok(mut logger) = DEFAULT_LOGGER.lock() {
        logger.write(message.to_string());
    }
}