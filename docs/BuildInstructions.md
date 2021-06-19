# How can I build/run torq?

### Install rust and cargo
- Linux: https://www.rust-lang.org/tools/install
- Macos: https://www.rust-lang.org/tools/install
- Windows: https://forge.rust-lang.org/infra/other-installation-methods.html

```sh
> git clone https://github.com/torq-lang/torqc
> cd torqc
> cargo run # If you want a debug build
> cargo run --release # If you want a release build
```

## If you want to install Torqc compiler
### Build from source 
```sh
> cargo install --path cli
```
#### NOTE: Other installation steps are yet to be created and hence after they are created we will link them up here
