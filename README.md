xcrust
===

[![Build Status](https://travis-ci.org/wilsonlab/xcrust.svg?branch=master)](https://travis-ci.org/wilsonlab/xcrust)

An experimental rewrite of mwl code in [rust](https://rust-lang.org).

If [coreutils can do it](https://github.com/uutils/coreutils), so can we!


Projects we will try to rewrite:
---

 - extract
 - spikeanal/popanal
 - behav
 - xclust
 
For each project, we will test our work using mwsoft64 as a reference. Let's
continue using that repository to host test data, rather than loading up
this repository (except for very small files to test serialization).


Usage
---

Don't use it yet - there is actually no code here, it's a placeholder :)


Building
---

We manage dependencies with [nix](https://nixos.org/nix), so this is the easiest
way to build the project.

Install nix if you don't have it already:

``` shell
curl https://nixos.org/nix/install | sh
```

Then enter the nix shell, where you can build the project:

``` shell
nix-shell
cargo run --bin xcrust-hello
```


Project organization
---

Shared code (the standard definition of a spike, LFP trace, etc) lives
in the xcrust library in `src/`. Invididual executables live in directories
under `src/bin/<EXECUTABLE>/main.rs` or `src/bin/<EXECUTABLE>.rs`.

Contributing
---

Please run `cargo fmt` before submitting pull requests, and if you have
a pull request with many commits that don't logically need to be separated,
flatten them to a smaller number with `git rebase`.

Contributions are greatly appreciated!
