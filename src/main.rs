use std::env;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = match args.as_slice() {
        [_, path] => path,
        _ => panic!("Usage: {} <path>", env::args().next().unwrap()),
    };

    let mut watcher = fsevent_dump::FsEventWatcher::new();

    watcher.watch(Path::new(path));

    unreachable!();
}
