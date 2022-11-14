use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::{use_effect, html};
use yew_router::prelude::{use_history, History, Location};
use yew::{function_component, Properties};
use crate::presentation::routing::{Page, Route};

fn next_page_id(code: u32, page_id: u32, last_page_id: u32) -> u32 {
    if page_id == 0 {
        return 1;
    }

    // space or right arrow
    if code == 32 || code == 39 {
        if page_id + 1 > last_page_id {
            return page_id;
        }
        return page_id + 1;
    }

    // left arrow
    if code == 37 && page_id > 1 {
        return page_id - 1;
    }

    page_id
}

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub slide_count: usize,
}

#[function_component(Keyboard)]
pub fn keyboardComponent(props: &KeyboardProps) -> Html {
    let last_page_id = props.slide_count as u32;

    let history = use_history().unwrap();

    // fixing ids so they don't go over or under the slide count
    match history.location().query::<Page>() {
        Ok(page) => {
            if page.id > last_page_id {
                match history.push_with_query(Route::Home, Page{ id: last_page_id }) {
                    Ok(_) => (),
                    Err(_) => gloo_console::log!("error, setting overflowed id"),
                };
            }
        },
        Err(_) => {
            match history.push_with_query(Route::Home, Page{ id: 1 }) {
                Ok(_) => (),
                Err(_) => gloo_console::log!("error, setting initial page"),
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
                    let next_page = next_page_id(event.key_code(), page.id, last_page_id);
                    if next_page == page.id {
                        return;
                    }

                    match history.push_with_query(Route::Home, Page{ id: next_page }) {
                        Ok(_) => (),
                        Err(_) => gloo_console::log!("error, push_with_query"),
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