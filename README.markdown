neovim-cmd
==========

Send commands to [neovim](https://neovim.io/) from `:terminal`.

This project is a way for me to learn rust and experiment with
the neovim API. It is in early development stage and not ready for production.

It is heavily inspired by [neovim-remote](https://github.com/mhinz/neovim-remote)
which has more features, and probably less bugs.


Usage
-----

```
USAGE:
    neovim-cmd [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cd
    edit
    help      Prints this message or the help of the given subcommand(s)
    rename
```


Features
--------

Use it as your commit message editor:

    ```sh
    git config core.editor '~/bin/neovim-cmd edit --wait'
    ```

Use it to synchronize `$PWD` with `:tchdir`:

    ```sh
    function cd() {
        builtin cd "$@";
        # do a vim :tcd if we managed to cd and we are withing neovim
        if [[ -n ${NVIM_LISTEN_ADDRESS} ]]; then
            ~/bin/neovim-cmd cd "$@"
        fi
    }
    export cd
    ```

Inspired from [Yazgoo's blog post](http://yazgoo.github.io/blag/neovim/terminal/multiplexer/tmux/2017/11/29/neovim-one-week-without-tmux.html)


Installation
------------

1. Install the rust toolchain with [`rustup.rs`](https://rustup.rs/)

    ```sh
    curl https://sh.rustup.rs -sSf | sh
    ```

2. Clone the source code:

    ```sh
    git clone https://github.com/Makohoek/neovim-cmd neovim-cmd
    cd neovim-cd
    ```

3. Make sure you have the latest stable version of `cargo` and friends:

    ```sh
    rustup override set stable
    rustup update stable
    ```

4. Build for release:

    ```sh
    cargo build --release
    ```

5. Copy the binary to a folder in your `$PATH`:

    ```sh
    cp target/release/neovim-cmd ~/bin
    ```


References
----------
- [neovim-remote](https://github.com/mhinz/neovim-remote)
- [neovim-lib](https://github.com/daa84/neovim-lib)
- [Yazgoo's blog post about :cd](http://yazgoo.github.io/blag/neovim/terminal/multiplexer/tmux/2017/11/29/neovim-one-week-without-tmux.html)
