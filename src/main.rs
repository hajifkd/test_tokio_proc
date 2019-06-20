extern crate tokio_process;

use std::path::Path;
use std::process::Command;
use tokio_process::CommandExt;

fn main() {
    let file_name = "foo";
    let foo = Path::new(file_name);
    if foo.exists() {
        std::fs::remove_file(foo).unwrap();
    }

    let _cmd = Command::new("touch").arg(file_name).spawn_async().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(!foo.exists());
}
