extern crate console_error_panic_hook;

use std::panic;

use leptos::*;
use log::{info, Level};

use crate::greeting::Greeting;

mod greeting;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _ = console_log::init_with_level(Level::Debug);
    info!("Hello, console!");

    mount_to_body(|| view! {
        <div class="flex flex-col min-h-screen">
            <header class="flex justify-between items-center p-2 bg-slate-300 dark:text-slate-200 dark:bg-slate-900">
                <div class="text-lg">Tauri + Leptos + Tailwind Example</div>
                <div><a class="font-nerd nf-md-github text-3xl inline-block w-[22px]" href="https://github.com/anozaki/tauri-leptos-example" target="_blank" /></div>
            </header>
            <Greeting />
        </div>
    })
}