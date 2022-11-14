use yew::{function_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::presentation::keyboard::Keyboard;
use crate::presentation::routing::{switch, Route};
use crate::presentation::slides::{Slides, read_slides};

#[function_component(App)]
pub fn app() -> Html {
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