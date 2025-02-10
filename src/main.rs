use gloo_timers::future::TimeoutFuture;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_window_size;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let screen = use_window_size();
    let (values, set_values) = signal(vec![]);
    let mut width = vec![200, 300, 400, 500];
    let longest = width.iter().max().unwrap().clone();
    let height = ((screen.height.get() as i32) * 11/12) /width.iter().len() as i32;
    let mut lengths = vec![];

    for el in width.iter_mut() {
        if *el == longest {
            *el = 100 as i32;
            continue;
        } 
        *el = (*el * 100) / longest;
    }

    for el in width.iter() {
        lengths.push((*el, height));
    }

    let (width, _) = signal(lengths);

    spawn_local(async move{
        for el in width.get() {
            TimeoutFuture::new(500).await;
            set_values.write().push(el);
        }
    });



    view! {
        <ul class="w-11/12 h-11/12">
            <For
                each=move || values.get()
                key=|item| item.clone()
                children=|item| {
                    view! {
                        <li
                            class="bg-violet-500"
                            style=format!("width: {}%; height: {}px", item.0, item.1)
                        ></li>
                    }
                }
            />
        </ul>
    }
}
