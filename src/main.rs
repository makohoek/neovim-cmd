extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};
use std::env;

// TODO: replace by clap crate
// https://rust-lang-nursery.github.io/rust-cookbook/app.html#ex-clap-basic
fn usage() {
    println!("usage: neovim_rsrc FILE");
}

fn main() {
    // collect argc, argv[]
    let args: Vec<String> = env::args().collect();

    let filename = match args.len() {
        2 => &args[1],
        _ => {
            usage();
            return;
        }
    };

    println!("file to open {}", filename);

    // first, check if we are within neovim's terminal (if neovim is running)
    let address = match env::vars().find(|&(ref key, ref _value)| key == "NVIM_LISTEN_ADDRESS") {
        // option.0 is the key (env variable name) option.1 is the value (env variable value)
        Some(option) => option.1,
        None => {
            eprintln!("This only works from within a neovim terminal");
            return;
        }
    };

    // create a session and start it
    println!("listening address {}", address);

    let mut session = Session::new_unix_socket(address).unwrap();
    session.start_event_loop();

    // create the nvim instance
    let mut nvim = Neovim::new(session);

    // send some commands
    nvim.command("vsplit").unwrap();
}

// nnoremap <buffer> <leader>pc :Dispatch cargo build<CR>
// nnoremap <buffer> <leader>pt :Dispatch cargo run<CR>
