use std::process::Command;

fn main() {
    // Shell out to perform the build.  In the future, the logic
    // to grab libcore could be done in rust in order to support
    // platforms without a posix shell
	Command::new("sh").arg("./build.sh").status().unwrap_or_else(|e| {
		panic!("failed to execute process: {}", e)
    });
}
