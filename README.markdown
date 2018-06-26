neovim-cmd
==========

Send commands to [neovim](https://neovim.io/) from `:terminal`.

This project is a way for me to learn rust and experiment with
the neovim API. It is in early development stage and not ready for production.

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


References
----------
- [neovim-remote](https://github.com/mhinz/neovim-remote)
- [neovim-lib](https://github.com/daa84/neovim-lib)
