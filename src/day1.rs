use gloo_timers::future::TimeoutFuture;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Day1() -> impl IntoView {
    let (values, set_values) = signal(vec![]);

    spawn_local(async move{
        let (mut left,mut right, _diff) = get_todo().await.unwrap();
        let max1 = left.iter().max().unwrap().clone();
        let max2 = right.iter().max().unwrap().clone();
        let max = max1.max(max2);

        left.sort();
        right.sort();

        for (idx, el) in left.iter().enumerate() {
            TimeoutFuture::new(1).await;
            if *el == max {
                set_values.write().push((100, (right[idx] * 100) /max));
            } else if right[idx] == max {
                set_values.write().push(((*el * 100)/max, 100));
            } else {
                set_values.write().push(((*el * 100)/max, (right[idx] * 100) /max));
            }
        }

        //TimeoutFuture::new(50).await;
        //*set_values.write() = vec![];
        //left.sort();
        //right.sort();
        //
        //for (idx, el) in left.iter().enumerate() {
        //    TimeoutFuture::new(2).await;
        //    log!("{:?}", (*el * 100)/max);
        //    if *el == max {
        //        set_values.write().push((100, (right[idx] * 100) /max, height));
        //    } else if right[idx] == max {
        //        set_values.write().push(((*el * 100)/max, 100, height));
        //    } else {
        //        set_values.write().push(((*el * 100)/max, (right[idx] * 100) /max, height));
        //    }
        //}
        //
        //log!("{}", values.get().len());
    });


    view! {
        <ul class="w-11/12 h-11/12">
            <For
                each=move || values.get()
                key=|item| item.clone()
                children=|item| {
                    view! {
                        <li class="relative">
                            <div
                                class="bg-green-500 absolute"
                                style=format!("width: {}%; height: {}px", item.0, 1)
                            ></div>
                            <div
                                class="bg-violet-500"
                                style=format!("width: {}%; height: {}px", item.1, 1)
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
