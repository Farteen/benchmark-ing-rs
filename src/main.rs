use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};
use std::process::Command;
use std::env;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, rt};

fn main() {
    // add sub option
    match unsafe { fork().expect("failed to fork process") } {
        ForkResult::Parent { child } => {
            println!("parent");
        }
        ForkResult::Child => {
            // Command::new("ls").spawn().unwrap();
            start_actix_web();
            std::process::exit(0);
        }
    }
}

fn start_actix_web() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let mut sys = rt::System::new("test");

    let srv = HttpServer::new(|| {
        App::new()
    })
        .bind("127.0.0.1:8080")?
        .run();
    let _ = tx.send(srv.clone());

    // run future
    sys.block_on(srv)
}