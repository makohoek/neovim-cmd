extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};
use std::env;

fn main() {
    // first, check if we are within neovim's terminal (if neovim is running)

    // retrieve the socket address
    let socket_address = env::vars().find(|&(ref key, ref _value)| key == "NVIM_LISTEN_ADDRESS");
    let session_address = socket_address.unwrap().1;

    // create a session and start it
    // TODO: use pattern matching here to handle the None case
    println!("listening address {}", session_address);
    let mut session = Session::new_unix_socket(session_address).unwrap();
    session.start_event_loop();

    // create the nvim instance
    let mut nvim = Neovim::new(session);

    // send some commands
    nvim.command("vsplit").unwrap();
}

// nnoremap <buffer> <leader>pc :Dispatch cargo build<CR>
// nnoremap <buffer> <leader>pt :Dispatch cargo run<CR>
