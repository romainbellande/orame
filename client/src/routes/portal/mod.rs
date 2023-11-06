use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::use_navigate;
use web_sys::{Event, SubmitEvent};

use reqwest;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[component]
pub fn Portal() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal("".to_string());
    let (is_register, set_is_register) = create_signal(false);
    let is_login = move || !is_register();
    let has_error = move || with!(|error| error.len() > 0);

    let on_email_input = move |ev: Event| {
        let value = event_target_value::<Event>(&ev);
        set_email(value);
    };

    let on_password_input = move |ev: Event| {
        let value = event_target_value::<Event>(&ev);
        set_password(value);
    };

    let send_auth = move |method: String, email: String, password: String| {
        let credentials = Credentials { email, password };

        console_log(format!("credentials: {:?}", credentials).as_str());

        spawn_local(async move {
            set_error("".to_string());
            let client = reqwest::Client::new();
            let response = client
                .post(format!("http://localhost:8080/auth/{}", method))
                .json(&credentials)
                .send()
                .await
                .map_err(|e| {
                    console_log(format!("error: {:?}", e).as_str());
                })
                .unwrap();
            console_log(format!("response: {:?}", response).as_str());
            let navigate = use_navigate();

            if response.status().is_success() {
                navigate("/", Default::default());
            } else {
                set_error("Invalid credentials".to_string());
            }
        });
    };

    let on_submit_login = move |ev: SubmitEvent| {
        ev.prevent_default();
        send_auth("login".to_string(), email(), password());
    };

    let on_submit_register = move |ev: SubmitEvent| {
        ev.prevent_default();
        send_auth("register".to_string(), email(), password());
    };

    view! {
        <section class="grid h-screen place-content-center bg-slate-900 text-slate-300">
            <div class="mb-10 text-center text-indigo-400">
                <h1 class="text-3xl font-bold tracking-widest">"ORAME"</h1>
                <p>"Beyond space"</p>
            </div>
            <Show when=is_register>
                <form on:submit=on_submit_register class="flex flex-col items-center justify-center space-y-6">
                    <input type="text" prop:value=email on:input=on_email_input id="email" name="email" placeholder="E mail" class="w-80 appearance-none rounded-full border-0 bg-slate-800/50 p-2 px-4 focus:bg-slate-800 focus:ring-2 focus:ring-orange-500" />
                    <input type="password" prop:value=password on:input=on_password_input id="password" name="password" placeholder="Password" class="w-80 appearance-none rounded-full border-0 bg-slate-800/50 p-2 px-4 focus:bg-slate-800 focus:ring-2 focus:ring-orange-500" />
                    <button id="submit" type="submit" class="rounded-full bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"Sign Up"</button>
                    <button on:click=move |_| set_is_register(false) class="text-indigo-400 hover:text-orange-500">"Already have an account ? Sign In"</button>
                </form>
            </Show>
            <Show when=is_login>
                <form on:submit=on_submit_login class="flex flex-col items-center justify-center space-y-6">
                    <input type="text" prop:value=email on:input=on_email_input id="email" name="email" placeholder="E mail" class="w-80 appearance-none rounded-full border-0 bg-slate-800/50 p-2 px-4 focus:bg-slate-800 focus:ring-2 focus:ring-orange-500" />
                    <input type="password" prop:value=password on:input=on_password_input id="password" name="password" placeholder="Password" class="w-80 appearance-none rounded-full border-0 bg-slate-800/50 p-2 px-4 focus:bg-slate-800 focus:ring-2 focus:ring-orange-500" />
                    <button id="submit" type="submit" class="rounded-full bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"Sign In"</button>
                    <button on:click=move |_| set_is_register(true) class="text-indigo-400 hover:text-orange-500">"Don't have an account ? Sign Up"</button>
                </form>
            </Show>
            <Show when=has_error>
                <p class="text-red-500">{error}</p>
            </Show>
      </section>
    }
}
