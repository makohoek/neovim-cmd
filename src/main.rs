extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

fn main() {
    let mut session = Session::new_unix_socket("/var/folders/bx/wys6xx8j0p9_bhmrxh26_kbr0000gq/T/nvimsE9jIV/0").unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);

    nvim.command("vsplit").unwrap();
}
