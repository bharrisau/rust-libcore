
#![feature(io)]
#![allow(unused_variables)]

use std::old_io::Command;

fn main() {
	let process = match Command::new("sh").arg("./build.sh").spawn() {
		Ok(p) => p,
		Err(e) => panic!("failed to execute process: {}", e),
	};
}