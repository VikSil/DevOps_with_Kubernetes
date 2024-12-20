use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::use_effect_with;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewTodo {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    pub todos: Vec<Todo>,
    pub image_url: String,
}

#[derive(PartialEq, Properties)]
struct TodoComponentProps {
    pub todo: Todo,
}

#[function_component(TodoComponent)]
fn todo_component(props: &TodoComponentProps) -> Html {
    let TodoComponentProps { todo } = props;
    html! {
        { todo.title.to_owned() }
    }
}

#[function_component(App)]
fn page_body() -> Html {
    let input_value = use_state(|| String::new());
    let refresh_flag = use_state(|| false);
    let todolist: UseStateHandle<Option<TodoList>> = use_state(|| None);

    let form_oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let form_onsubmit = {
        let input_value = input_value.clone();
        let refresh_flag = refresh_flag.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let new_todo = NewTodo {
                title: String::from(input_value.as_str()),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let todos_url: String = format!("http://localhost:3040/todos");
                let data_serialized = serde_json::to_string_pretty(&new_todo).unwrap();

                let _request = Request::post(&todos_url)
                    .header("Content-Type", "application/json")
                    .body(wasm_bindgen::JsValue::from(&data_serialized))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
            });
            input_value.set(String::from(""));
            refresh_flag.set(!*refresh_flag);
        })
    };

    {
        let todolist = todolist.clone();
        use_effect_with(refresh_flag, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let todos_url: String = format!("http://localhost:3040/todos");

                let fetched_todos: TodoList = Request::get(&todos_url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                todolist.set(Some(fetched_todos));
            });
        });
    }

    html! {
        <div>
            <div>
                <img src={
                    match todolist.as_ref() {
                        Some(tdl) => tdl.image_url.clone(),
                        None => String::from("https://fastly.picsum.photos/id/633/1200/1200.jpg?hmac=w3wSzGHuyT-aMKInisjPvciLC7gIgyXaBAeU7nzo-c4")
                    }
                } alt="Random picture" width="300" class="center-image" />
            </div>
            <div>
            <form onsubmit={form_onsubmit}>
                <input
                    type="text" maxlength = "140"
                    value={(*input_value).clone()}
                    oninput={form_oninput}/>
                <input type="submit" value="Create TODO" />
            </form>
            </div>
            <div>
            <ul>
                {
                match todolist.as_ref() {
                    Some(tdl) => tdl
                        .todos.iter().map(|todo| {
                            html! {
                                <li>
                                <TodoComponent todo={todo.clone()}/>
                                </li>
                            }
                        })
                        .collect(),
                    None =>{html! {<>{"No data yet"}</>} },
                }
            }
            </ul>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
