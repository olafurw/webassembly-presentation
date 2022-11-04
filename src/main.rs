use serde::{Serialize, Deserialize};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

fn slide1() -> Html {
    html! {
        <>
            <p>{ "HEY, WORLD, WHAT IS UP?" }</p>
        </>
    }
}

fn slide2() -> Html {
    html! {
        <>
            <p>{ "HEY, WORLD, WHAT IS UP?" }</p>
            <p>{ "Now we have the Emscripten compiler installed in our system. "}</p>
        </>
    }
}

fn slide3() -> Html {
    html! {
        <>
            <p>{ "HEY, WORLD, WHAT IS UP?" }</p>
            <p>{ "Now we have the Emscripten compiler installed in our system. "}</p>
            <p>{ "Time for the time honored tradition of the hello world example."}</p>
        </>
    }
}

#[function_component(Slides)]
fn slides() -> Html {
    let history = use_history().unwrap();
    let page_id = match history.location().query::<Page>() {
        Ok(page) => page.id,
        Err(_) => 1,
    };

    if page_id == 1 {
        slide1()
    } else if page_id == 2 {
        slide2()
    } else {
        slide3()
    }
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
        <div></div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <KeyboardHack />
            <Switch<Route> render={Switch::render(switch)} />
            <Slides />
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}