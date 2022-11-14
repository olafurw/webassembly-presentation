use gloo_net::http::Request;
use yew::{function_component};
use yew::prelude::*;
use yew_router::prelude::*;
use regex::Regex;

mod presentation;
use crate::presentation::keyboard::Keyboard;
use crate::presentation::routing::{switch, Route, Page};

fn parse_slides(text: &str) -> Vec<String> {
    let pattern = Regex::new(r"\n(@@@[ \t]*)\n").unwrap();
    let slides: Vec<String> = pattern.split(text).map(|x| x.trim().to_string()).collect();
    slides
}

fn read_slides(slide_state: &UseStateHandle<Vec<String>>) {
    let slide_state = slide_state.clone();
    let f = async move {
        let request = Request::get("http://127.0.0.1:8000/slides.md").send().await;
        match request {
            Ok(response) => {
                let text = response.text().await;
                match text {
                    Ok(body) => { 
                        slide_state.set(parse_slides(&body));
                    },
                    Err(err) => { gloo_console::log!(format!("error, getting response text: {err}")); }
                };
            },
            Err(err) => { gloo_console::log!(format!("error, getting request: {err}")); }
        };
    };
    wasm_bindgen_futures::spawn_local(f);
}

fn extract_css_class(slide_text: &str) -> (String, String) {
    if slide_text.starts_with('@') {
        let pattern = Regex::new(r"(@[a-zA-Z0-9_\-]+\n)").unwrap();
        let cap = pattern.captures(slide_text).unwrap();
        
        let css_name = cap.get(1).unwrap().as_str().trim();
        let css_name_len = css_name.len();
        return (String::from(&css_name[1..]), String::from(&slide_text[css_name_len..]));
    }

    (String::new(), String::from(slide_text))
}

fn slide(slide_text: &str) -> Html {
    let (css_name, text) = extract_css_class(slide_text);

    let parser = pulldown_cmark::Parser::new(text.as_str());

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_class_name(&css_name);
    div.set_inner_html(&html_output);

    Html::VRef(div.into())
}

#[function_component(Slides)]
fn slides() -> Html {
    let slide_list = match use_context::<Vec<String>>() {
        Some(s) => { s },
        None => { 
            gloo_console::log!("error, fetching slide context.");
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
            <Keyboard slide_count={slide_list.len()} />
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