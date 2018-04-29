# gpm
[![Crates.io](https://img.shields.io/crates/v/vips-sys.svg)](https://crates.io/crates/vips-sys)
[![Build Status](https://travis-ci.org/elbaro/vips-sys.svg?branch=master)](https://travis-ci.org/elbaro/vips-sys)

[Documentation](https://docs.rs/gpm)

General package manager [WIP]

GPM does several things:
1. Provides a thin wrapper a round system package managers (apt-get, pacman, ..)

```rust
let manager = Pacman::new().unwrap();
manager.is_installed("pacman")?;
manager.sync()?;
manager.install("opencv")?;
manager.install("ffmpeg")?;
```

2. Provides `Component` class on which you can build your own object-oriented dotfile.

```rust
// define zsh-autosuggestion
component!(zsh_autosuggestion, || apt_get.install("zsh-autosuggestion"), zsh);

// define alacritty
...

// define my_zsh_package
component!(my_zsh_package, || {}, zsh_autosuggestion, alacritty, color_scheme, font, ..)


// recursively install missing components
apt_get.sync()?;
my_zsh_package.install()?;
```

3. Provides system platform / resources as `Component`

```
// gpm::Gpu::install() always fail
component!(Cuda9, || {..}, gpm::Gpu)
component!(tensorflow_gpu, || { pip.install(""); },gpm::Cuda9, gpm::MMX, ..)

// fail if graphics card is not found.
tensorflow_gpu.install();

gpm::Gpu::is_installed();
gpm::Windows::is_installed();
gpm::x64::is_installed();
```

This code is a proof of concept.

## Design to-do
- Currently to use system package managers, you need to run your entire binary with `sudo`. You may not want `sudo` previlege in the other part of the program. Also this doesn't work well with `pacaur`.
- Version specification should be implemented.
- interactive / non-interactive mode
