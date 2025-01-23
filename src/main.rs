use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>Square</div>
        <button on:click=move |_| *set_count.write() += 1>"Click me: " {count}</button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
