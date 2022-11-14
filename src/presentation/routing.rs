use serde::{Serialize, Deserialize};
use yew_router::prelude::{Routable};
use yew::prelude::{Html, html};

#[derive(Serialize, Deserialize)]
pub struct Page {
  pub id: u32
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <>
            </>
        },
    }
}