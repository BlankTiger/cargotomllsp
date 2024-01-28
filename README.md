# <p align="center">cargotomllsp 🔧</p>

This LSP server will spare you from going over to crates.io/docs.rs/... to find whatever package version is the latest one. Inspired by some videos of people having this functionality in vsc\*de. Can't let those pesky vsc\*de soyboy developers get away with having more features (support for vsc\*de will maybe be added in the future)...

## Supported features 🧰

Currently, server is capable of supplying:
- latest version of the crate
- feature names

## Usage 🛠️

In the future this will maybe be available through [Mason](https://github.com/williamboman/mason.nvim). For now, you have to have Rust installed, and then you have to install by running:

```bash
$ git clone https://github.com/BlankTiger/cargotomllsp
$ cd cargotomllsp
$ cargo install --path .
```

### Neovim ⚡

WIP. But currently if you want a quick way to attach it to a buffer, then run the code from `attach_lsp.lua` in a buffer you want to attach to. A quick way to run lua code in neovim is to use the following command:

```lua
luafile <path-to-file>
```

## Logging

Server should be outputting logs to `$HOME/.local/state/cargotomllsp.log`.
