# Tauri + Leptos Example

This repository provides an example setup of Tauri/Leptos.

## About

This repository showcases the following features:

* Rust ToolChain
    * Using the `nightly` build for Leptos (optional, but recommended).
* [Tauri](https://tauri.app) backend component is in `backend`
  * Exposing a `greet` API
* [Leptos](https://leptos.dev) ui component is in `ui`
  * Trunk setup [Tailwind](https://tailwindcss.com/)
    * Setup to compile tailwind
    * Setup to include custom font file
  * Custom font ([Patched JetBrains Mono](https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/JetBrainsMono))
    * Custom css for tailwind
  * Example of calling Tauri service using [tauri-sys](https://github.com/JonasKruckenberg/tauri-sys)
  * Console Log usage
    * Debug Level with [console-log](https://github.com/iamcodemaker/console_log)
    * Panic handler [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook)

## Pre-Requisite

For each of these dependency, follow the official documentation for more detail.

### [Rust](https://www.rust-lang.org/)

The entire project is based on Rust 😉. Follow the instruction on the [Rust getting started](https://www.rust-lang.org/learn/get-started)
page to install the basic tools. 

### [Trunk](https://trunkrs.dev/) and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

This example uses `Trunk` to bundle the WASM web application for both release and development. During development,
the artifacts will be served and recompiled by `Trunk`.

```bash
cargo install trunk
cargo install wasm-bindgen-cli
```

### [Tauri](https://tauri.app)

`Tauri` will manage the development, building and bundling of your Tauri application. In order to use Tauri, you will
need to install the Tauri CLI.

```bash
cargo install tauri-cli
```

## Running

Running the application involves two primary modes: development and production.

### Development

In development mode, the application runs in a live-reload environment, enabling real-time feedback and testing. 
When you execute the following command, it initiates the build process and starts up the Tauri application in
development mode.

This mode provides the following functionalities:

* **Backend/Service Component Reloading:** Tauri manages service components, automatically reloading them as changes
  are made.
* **Client-Side Reloading:** Trunk handles the reloading of the client-side interface, ensuring that updates are 
  immediately reflected in the application.

To start in development mode, use the following command:

```bash
cargo tauri dev
```

### Release

Release mode is designed for building the final version of your application, optimized for distribution and deployment.
In this mode, the application is compiled into an optimized, standalone binary, suitable for end-user use.

The release build provides:

* **Compilation and Optimization:** Application is compiled into an executable format, with optimization for performance and size.
* **Packaging:** The application is bundled with all necessary artifacts, ready for distribution.

To build your application for release, use the following command:

```bash
cargo tauri build
```

After building, the output files will be located in the `target/release/bundle` directory. The output will depend on the OS you
are using to build.
