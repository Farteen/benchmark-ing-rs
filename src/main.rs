use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};
use std::process::Command;

fn main() {
    match unsafe { fork().expect("failed to fork process") } {
        ForkResult::Parent { child } => {
            println!("parent");
        }
        ForkResult::Child => {
            Command::new("ls").spawn().unwrap();
            std::process::exit(0);
        }
    }
}
