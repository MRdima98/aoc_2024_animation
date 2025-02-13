use gloo_timers::future::TimeoutFuture;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_window_size;

#[component]
pub fn Day1() -> impl IntoView {
let screen = use_window_size();
    let (values, set_values) = signal(vec![]);
    let (test, set_test) = signal(vec![]);
    let mut left = vec![200, 300, 400, 500];
    let mut right = vec![210, 250, 450, 470];
    let mut diff = vec![210, 250, 450, 470];

    let val = 5;
    spawn_local(async move{
        let (one, _, _) = get_todo().await.unwrap();
        set_test.write().push(one[0].clone());
        set_test.write().push(val);
    });

    log!("{:?}", test.get());

    let max1 = left.iter().max().unwrap().clone();
    let max2 = left.iter().max().unwrap().clone();
    let max = max1.max(max2);
    let height = ((screen.height.get() as usize) * 11/12) /left.iter().len() as usize;
    let mut lists = vec![];

    for el in left.iter_mut() {
        if *el == max {
            *el = 100 as usize;
            continue;
        } 
        *el = (*el * 100) / max;
    }

    for el in right.iter_mut() {
        if *el == max {
            *el = 100 as usize;
            continue;
        } 
        *el = (*el * 100) / max;
    }

    let mut i = 0;
    for el in left.iter() {
        lists.push((*el, right[i],height ));
        i += 1;
    }

    let (width, _) = signal(lists);
    let (text, setText) = signal(" empty".to_string());

    spawn_local(async move{
        for el in width.get() {
            TimeoutFuture::new(200).await;
            set_values.write().push(el);
        }
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
pub async fn get_todo() -> Result<(Vec<usize>, Vec<usize>, Vec<usize>), ServerFnError> {
    let mut left = vec![];
    let mut right = vec![];
    let mut diff = vec![];

    let left_file = include_str!("../data/day1_left");
    for el in left_file.lines() {
        left.push(el.parse::<usize>().unwrap());
    }

    let right_file = include_str!("../data/day1_right");
    for el in right_file.lines() {
        right.push(el.parse::<usize>().unwrap());
    }

    let diff_file = include_str!("../data/day1_diff");
    for el in diff_file.lines() {
        diff.push(el.parse::<usize>().unwrap());
    }

    Ok((left, right, diff))
}
