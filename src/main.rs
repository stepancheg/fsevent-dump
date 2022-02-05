use std::path::Path;
use listen_notify::{Watcher, RecursiveMode};

fn main() {
    // Automatically select the best implementation for your platform.
    let mut watcher = listen_notify::FsEventWatcher::new();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("/Users/nga/fbsource"), RecursiveMode::Recursive);

    unreachable!();
}
