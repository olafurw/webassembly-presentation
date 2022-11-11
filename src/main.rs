use serde::{Serialize, Deserialize};
use gloo::events::EventListener;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;
use regex::Regex;

fn parse_slides(text: &str) -> Vec<String> {
    let pattern = Regex::new(r"\n(@@@[ \t]*)\n").unwrap();
    let slides: Vec<String> = pattern.split(text).map(|x| x.trim().to_string()).collect();
    slides
}

fn read_slides(slide_state: &UseStateHandle<Vec<String>>) {
    let slide_state = slide_state.clone();
    let f = async move {
        let request = Request::get("http://127.0.0.1:8080/slides.md").send().await;
        match request {
            Ok(response) => {
                let text = response.text().await;
                match text {
                    Ok(body) => { 
                        slide_state.set(parse_slides(&body));
                    },
                    Err(err) => { gloo_console::log!(format!("Error getting response text: {err}")); }
                };
            },
            Err(err) => { gloo_console::log!(format!("Error in request: {err}")); }
        };
    };
    wasm_bindgen_futures::spawn_local(f);
}

fn slide(slide_text: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(slide_text);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_inner_html(&html_output);

    Html::VRef(div.into())
}

#[function_component(Slides)]
fn slides() -> Html {
    let slide_list = match use_context::<Vec<String>>() {
        Some(s) => { s },
        None => { 
            gloo_console::log!("Error fetching slide context.");
            return html!{ <></> };
        }
    };

    if slide_list.is_empty() {
        return html!{ <></> };
    }

    let history = use_history().unwrap();
    let page_id: usize = match history.location().query::<Page>() {
        Ok(page) => {
            let slide_count = slide_list.len();
            let page_id = page.id as usize;
            if page_id >= slide_count { slide_count } else { page_id }
        },
        Err(_) => 1,
    };

    slide(&slide_list[page_id - 1])
}

#[derive(Serialize, Deserialize)]
struct Page {
  id: u32
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <>
            </>
        },
    }
}

fn next_page_id(code: u32, page_id: u32) -> u32 {
    if page_id == 0 {
        return 1;
    }

    // space or right arrow
    if code == 32 || code == 39 {
        return page_id + 1;
    }

    // left arrow
    if code == 37 && page_id > 1 {
        return page_id - 1;
    }

    page_id
}

#[function_component(KeyboardHack)]
fn keyboardHack() -> Html {
    let history = use_history().unwrap();
    match history.location().query::<Page>() {
        Ok(_) => (),
        Err(_) => {
            match history.push_with_query(Route::Home, Page{ id: 1 }) {
                Ok(_) => (),
                Err(_) => gloo_console::log!("initial page error"),
            };
        },
    };

    use_effect(move || {
        // Attach a keydown event listener to the document.
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // space, left arrow, right arrow
            if !&[32, 37, 39].contains(&event.key_code()) {
                return;
            }

            match history.location().query::<Page>() {
                Ok(page) => {
                    match history.push_with_query(Route::Home, Page{ id: next_page_id(event.key_code(), page.id) }) {
                        Ok(_) => (),
                        Err(_) => gloo_console::log!("push_with_query error"),
                    };
                },
                Err(_) => gloo_console::log!("error, no page"),
            };
        });

        // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
        // dropped, `gloo` detaches it from the DOM. So it's important to do _something_, even if it's just dropping it.
        || drop(listener)
    });

    html! {
        <></>
    }
}

#[function_component(App)]
fn app() -> Html {
    let slide_list = use_state_eq(Vec::<String>::new);
    read_slides(&slide_list);
    if slide_list.is_empty() {
        return html! { <></> };
    }

    html! {
        <>
        <BrowserRouter>
            <KeyboardHack />
            <Switch<Route> render={Switch::render(switch)} />
            <ContextProvider<Vec<String>> context={(*slide_list).clone()}>
                <Slides />
            </ContextProvider<Vec<String>>>
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}