use std::path::Path;
use std::thread;
use std::time::Duration;
use listen_notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Event};
use listen_notify::event::Flag;

fn main() -> listen_notify::Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = listen_notify::recommended_watcher(|res: Result<Event>| {
        match res {
           Ok(event) => {
               if format!("{:?}", event).contains(".hg") {
                   return;
               }
               if event.attrs.flag() == Some(Flag::Rescan) {
                   // panic!("obtain stack trace: {:?}", event);
                   println!("rescan {:?}", event);
               }
               // println!("event: {:?}", event)
           },
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("/Users/nga/fbsource"), RecursiveMode::Recursive)?;

    loop {
        thread::sleep(Duration::from_secs(1));
    }
    println!("done");
    Ok(())
}
