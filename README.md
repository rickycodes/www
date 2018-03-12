### my website <a href='https://ricky.codes'>ricky.codes</a>  
[![Build Status](https://travis-ci.org/rickycodes/www.svg?branch=master)](https://travis-ci.org/rickycodes/www) [![experimental](http://badges.github.io/stability-badges/dist/experimental.svg)](http://github.com/badges/stability-badges)

My personal website built with <a href='http://rust-lang.org/'>Rust</a> using <a href='https://github.com/koute/cargo-web'>cargo-web</a> and <a href='https://github.com/koute/stdweb'>stdweb</a>

<img src='screenshot.png' />

### build:  
you will need <a href='https://github.com/koute/cargo-web'>`cargo web`</a>

```sh
cargo web build --target=wasm32-unknown-unknown
```
I haven't tested other targets

### you should see something like:  
```sh
warning: debug builds on the wasm32-unknown-unknown are currently totally broken
         forcing a release build
    Finished release [optimized] target(s) in 0.0 secs
```
### web server:
```sh
cargo web start --target=wasm32-unknown-unknown
```
### you should see something like:  
```sh
warning: debug builds on the wasm32-unknown-unknown are currently totally broken
         forcing a release build
    Finished release [optimized] target(s) in 0.0 secs

If you need to serve any extra files put them in the 'static' directory
in the root of your crate; they will be served alongside your application.
You can also put a 'static' directory in your 'src' directory.

Your application is being served at '/rickycodes.js'. It will be automatically
rebuilt if you make any changes in your code.

You can access the web server at `http://[::1]:8000`.
```
