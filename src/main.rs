extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};
use std::env;

fn main() {
    let socket_address = env::vars().find(|&(ref key, ref _value)| key == "NVIM_LISTEN_ADDRESS");

    // TODO: use pattern matching here to handle the None case
    let session_address = socket_address.unwrap().1;
    println!("listening address {}", session_address);

    let mut session = Session::new_unix_socket(session_address).unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);

    nvim.command("vsplit").unwrap();
}
