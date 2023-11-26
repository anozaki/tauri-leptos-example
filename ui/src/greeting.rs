use leptos::*;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use log::info;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Greeting() -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();

    let greet_action = create_action(|args: &String| {
        let args = args.to_owned();
        async move {
            tauri::invoke::<_, String>("greet", &GreetArgs { name: &args }).await.unwrap()
        }
    });
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("input text").value();
        greet_action.dispatch(value);
    };

    view! {
        <div class="flex flex-col flex-grow justify-items-center items-center h-full">
            <div class="m-auto">
                {move || {
                    if let Some(val) = greet_action.value().get() {
                        {view! {
                            <div class="text-6xl text-center">
                                <div class="my-10">Greetings from <span class="font-nerd nf-md-language_rust" /></div>
                                <div>{val}</div>
                                <div><span class="font-nerd nf-fa-handshake_o" /></div>
                            </div>
                        }}
                    } else {
                        {view! {
                            <div>
                                <div class="text-6xl my-5">Hello, what is your name?</div>
                                <div>
                                    <form on:submit=on_submit>
                                        <input class="align-middle text-5xl" type="text" node_ref=input_element />
                                        <input class="mx-10 bg-slate-300 p-2 rounded text-2xl align-middle" type="submit" value="submit" />
                                    </form>
                                </div>
                            </div>
                        }}
                    }
                }}

            </div>
        </div>
    }
}