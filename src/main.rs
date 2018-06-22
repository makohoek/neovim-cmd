extern crate clap;
extern crate neovim_lib;

use clap::App;
use neovim_lib::{Neovim, NeovimApi, Session, Handler, Value};
use std::env;
use std::{thread, time};


impl Handler for NeovimHandler{
    fn handle_notify(&mut self, _name: &str, _args: Vec<Value>) {
        println!("event: {}", _name);

    }

    fn handle_request( &mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        Err(Value::from("not implemented"));
    }
}

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
    let mut command = String::from("edit");
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

    let curbuf = nvim.get_current_buf().unwrap();
    println!("buffer name: {}", curbuf.get_name(&mut nvim).unwrap());

    let attach_ok = curbuf.attach(&mut nvim, false, [].to_vec()).unwrap();

    if ! attach_ok {
        eprintln!("could not attach to buffer");
        return;
    }

    loop {
        // read the communication channel for updates
        //
        thread::sleep(time::Duration::from_millis(2000));
    }

    // now wait on the buffer to be deleted
    // NAIVE implementation:
    // wait while there is still a COMMIT_EDITMSG buffer
    // let mut buf_deleted = false;
    // while ! buf_deleted {
    //     let mut buf_found = false;
    //     let buffers = nvim.list_bufs().unwrap();
    //     for buf in buffers {
    //         let bufname = buf.get_name(&mut nvim).unwrap();
    //         println!("name={}", bufname);
    //         if bufname.find("setup.sh") != None {
    //             buf_found = true;
    //         }
    //     }
    //     if buf_found == false {
    //         buf_deleted = true;
    //     }
    //     let output = nvim.command_output("ls").unwrap();
    //     println!("output {}", output);

    //     thread::sleep(time::Duration::from_millis(2000));
    // }
}

// nnoremap <buffer> <leader>pc :Dispatch cargo build<CR>
// nnoremap <buffer> <leader>pt :Dispatch cargo run<CR>
// nnoremap <buffer> <leader>pf :Dispatch cargo fmt<CR>
