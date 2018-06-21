extern crate clap;
extern crate neovim_lib;

use clap::App;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::env;

fn main() {
    // https://rust-lang-nursery.github.io/rust-cookbook/app.html#ex-clap-basic
    let matches = App::new("Neovim Remote Control")
        .version("0.0.1")
        .author("Mattijs Korpershoek <mattijs.korpershoek@gmail.com>")
        .about("Control neovim from the :terminal")
        .arg_from_usage("<file> 'File to edit'")
        .get_matches();

    let filename = matches.value_of("file").unwrap();

    println!("file to open {}", filename);

    // build the command to send to neovim
    let mut command = String::from("tabedit");
    command = command + " ";
    command = command + filename;

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
    nvim.command(&command).unwrap();
}

// nnoremap <buffer> <leader>pc :Dispatch cargo build<CR>
// nnoremap <buffer> <leader>pt :Dispatch cargo run<CR>
