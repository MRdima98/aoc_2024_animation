use day1::Day1;
use gloo_timers::future::TimeoutFuture;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::{components::*, path};
use leptos::task::spawn_local;
use leptos_use::use_window_size;

pub mod day1;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Day1 />
            // <Route path=path!("/users") view=Users />
            </Routes>
        </Router>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! { "hello" }
}

