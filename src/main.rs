use gloo_timers::future::TimeoutFuture;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::task::spawn_local;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    //let (right, set_right) = signal("blue".to_string());
    //let mut values = vec![100, 200, 300];
    let (values, set_values) = signal(vec![100]);
    let (width, _) = signal(vec![200, 300, 400, 500]);

    spawn_local(async move{
        for el in width.get() {
            //sleep(Duration::from_millis(200)).await;
            TimeoutFuture::new(500).await; // Wait 200ms
            set_values.write().push(el);
            //Timeout::new(1000, move || {
            //    //*set_color.write() = "blue".to_string();
            //    //*set_pos.write() = "20px"
            //    set_values.write().push(el);
            //})
            //.forget();
        }
    });



    view! {
        <ul>
            <For
                each=move || values.get()
                key=|item| item.clone()
                children=|item| {
                    view! {
                        <li
                            class="bg-violet-500"
                            style=format!("width: {}px; height: {}px", item, 10)
                        ></li>
                    }
                }
            />
        </ul>
    }
}
