use std::env;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = match args.as_slice() {
        [_, path] => path,
        _ => panic!("Usage: {} <path>", env::args().next().unwrap()),
    };

    // Automatically select the best implementation for your platform.
    let mut watcher = listen_notify::FsEventWatcher::new();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new(path));

    unreachable!();
}
