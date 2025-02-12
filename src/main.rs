use gloo_timers::future::TimeoutFuture;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::{components::*, path};
use leptos::task::spawn_local;
use leptos_use::use_window_size;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Day1 />
                <Route path=path!("/users") view=Users />
            </Routes>
        </Router>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! { "hello" }
}

#[component]
fn Day1() -> impl IntoView {
let screen = use_window_size();
    let (values, set_values) = signal(vec![]);
    let mut list1 = vec![200, 300, 400, 500];
    let mut list2 = vec![210, 250, 450, 470];
    let max1 = list1.iter().max().unwrap().clone();
    let max2 = list1.iter().max().unwrap().clone();
    let max = max1.max(max2);
    let height = ((screen.height.get() as i32) * 11/12) /list1.iter().len() as i32;
    let mut lists = vec![];

    for el in list1.iter_mut() {
        if *el == max {
            *el = 100 as i32;
            continue;
        } 
        *el = (*el * 100) / max;
    }

    for el in list2.iter_mut() {
        if *el == max {
            *el = 100 as i32;
            continue;
        } 
        *el = (*el * 100) / max;
    }

    let mut i = 0;
    for el in list1.iter() {
        lists.push((*el, list2[i],height ));
        i += 1;
    }

    let (width, _) = signal(lists);
    let (text, setText) = signal(" empty".to_string());

    spawn_local(async move{
        for el in width.get() {
            TimeoutFuture::new(200).await;
            set_values.write().push(el);
        }

        *setText.write() = get_todo().await.unwrap();
    });

    view! {
        <div class="text-white">Num: {text}</div>
        <ul class="w-11/12 h-11/12">
            <For
                each=move || values.get()
                key=|item| item.clone()
                children=|item| {
                    view! {
                        <li class="relative">
                            <div
                                class="bg-green-500 absolute"
                                style=format!("width: {}%; height: {}px", item.0, item.2)
                            ></div>
                            <div
                                class="bg-violet-500"
                                style=format!("width: {}%; height: {}px", item.1, item.2)
                            ></div>
                        </li>
                    }
                }
            />
        </ul>
    }
}

#[server(GetTodo, "/api")]
pub async fn get_todo() -> Result<String, ServerFnError> {
    Ok("five".to_string())
}
