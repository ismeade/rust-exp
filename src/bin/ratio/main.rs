use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let pb = ProgressBar::new(100);
    pb.enable_steady_tick(Duration::from_millis(100));
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(50));
        pb.inc(2);
    }
    pb.finish_with_message("done");

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    let name = "app_001";
    // ... do some work
    bar.set_prefix("[0/1]");
    bar.set_message(format!("{} download...", name));
    thread::sleep(Duration::from_secs(5));
    bar.set_message(format!("{} unzip...", name));
    thread::sleep(Duration::from_secs(3));
    bar.set_message(format!("{} start...", name));
    thread::sleep(Duration::from_secs(3));
    bar.finish_with_message(format!("{} complete!", name));

    let mp = MultiProgress::new();
    mp.add(bar);
}