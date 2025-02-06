use gloo_timers::callback::Timeout;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (left, set_left) = signal("red".to_string());
    let (right, set_right) = signal("blue".to_string());
    let (pos, set_pos) = signal("50px");
    // let (values, set_values) = create_signal::<Vec<usize>>(vec![1, 2, 3]);
    let values = vec![100, 200, 300];

    Timeout::new(1000, move || {
        //*set_color.write() = "blue".to_string();
        *set_pos.write() = "20px"
    })
    .forget();

    view! {
        <Stylesheet id="leptos" href="/style.css" />
        <div style=move || {
            format!("background-color: {}; position: fixed; top:0px; left:0px;", left.get())
        }>Square 1</div>
        <div style=move || {
            format!(
                "background-color: {}; position: fixed; top:0px; left:{};",
                right.get(),
                pos.get(),
            )
        }>Square 2</div>
        <button class="bg-gruvbox" on:click=move |_| *set_left.write() = "blue".to_string()>
            "Red"
        </button>

        <button on:click=move |_| *set_count.write() += 1>"Click me: " {count}</button>

        // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            // {values.into_iter()
            //     .map(|n| view! { <li class=move || n></li>})
            //     .collect::<Vec<_>>()}
            <For each=move|| values.clone() key=|item| item.clone()
                children=|item| view! {
                    <li class="bg-red-500" style=format!("width: {}px", item)>lollone</li>
                }
            />
        </ul>
    }
}
