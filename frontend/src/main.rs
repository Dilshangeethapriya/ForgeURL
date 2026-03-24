use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::{
    home::Home,
    dashboard::Dashboard,
    api_docs::ApiDocs,
    about::About,
    not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/api-docs")]
    ApiDocs,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home      => html! { <Home /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::ApiDocs   => html! { <ApiDocs /> },
        Route::About     => html! { <About /> },
        Route::NotFound  => html! { <NotFound /> },
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}