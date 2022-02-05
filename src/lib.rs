
pub use config::{Config, RecursiveMode};
pub use error::{Error, ErrorKind, Result};
pub use event::{Event, EventKind};
use std::path::Path;

pub use crate::fsevent::FsEventWatcher;
pub use poll::PollWatcher;

pub mod fsevent;

pub mod event;
pub mod poll;

mod config;
mod error;

pub trait EventHandler: Send + 'static {
    /// Handles an event.
    fn handle_event(&mut self, event: Result<Event>);
}

impl<F> EventHandler for F
where
    F: FnMut(Result<Event>) + Send + 'static,
{
    fn handle_event(&mut self, event: Result<Event>) {
        (self)(event);
    }
}

impl EventHandler for crossbeam_channel::Sender<Result<Event>> {
    fn handle_event(&mut self, event: Result<Event>) {
        let _ = self.send(event);
    }
}

impl EventHandler for std::sync::mpsc::Sender<Result<Event>> {
    fn handle_event(&mut self, event: Result<Event>) {
        let _ = self.send(event);
    }
}

/// Type that can deliver file activity notifications
///
/// Watcher is implemented per platform using the best implementation available on that platform.
/// In addition to such event driven implementations, a polling implementation is also provided
/// that should work on any platform.
pub trait Watcher {
    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode);

    fn configure(&mut self, _option: Config) -> Result<bool> {
        Ok(false)
    }
}
