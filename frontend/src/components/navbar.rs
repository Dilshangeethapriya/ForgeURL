use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

fn close_navbar() {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(collapse) = document.get_element_by_id("navbarNav") {
            let class = collapse.class_name();
            collapse.set_class_name(&class.replace(" show", "").replace("show ", "").replace("show", ""));
        }
        if let Some(toggler) = document.query_selector(".navbar-toggler").ok().flatten() {
            let _ = toggler.set_attribute("aria-expanded", "false");
        }
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    // close on outside click
    use_effect_with((), |_| {
        let closure = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(
            move |e: web_sys::MouseEvent| {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(navbar) = document.query_selector(".navbar").ok().flatten() {
                        if let Some(target) = e.target() {
                            if let Ok(node) = target.dyn_into::<web_sys::Node>() {
                                if !navbar.contains(Some(&node)) {
                                    close_navbar();
                                }
                            }
                        }
                    }
                }
            },
        ));

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();

        closure.forget();
        || ()
    });

    let on_link_click = Callback::from(|_: MouseEvent| close_navbar());

    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-white sticky-top align-items-center">
            <div class="container">
                <a class="navbar-brand header-logo" href="/">
                    <img src="assets/images/ForgeURL-tr.png" alt="ForgeURL Logo" />
                </a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarNav"
                    aria-controls="navbarNav"
                    aria-expanded="false"
                    aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav ms-auto gap-3">
                        <li class="nav-item" onclick={on_link_click.clone()}>
                            <Link<Route> classes="nav-link" to={Route::Home}>
                                {"Home"}
                            </Link<Route>>
                        </li>
                        <li class="nav-item" onclick={on_link_click.clone()}>
                            <Link<Route> classes="nav-link" to={Route::Dashboard}>
                                {"Dashboard"}
                            </Link<Route>>
                        </li>
                        <li class="nav-item" onclick={on_link_click.clone()}>
                            <Link<Route> classes="nav-link" to={Route::ApiDocs}>
                                {"API Docs"}
                            </Link<Route>>
                        </li>
                        <li class="nav-item" onclick={on_link_click.clone()}>
                            <Link<Route> classes="nav-link" to={Route::About}>
                                {"About"}
                            </Link<Route>>
                        </li>
                        <li class="nav-item d-lg-none" onclick={on_link_click.clone()}>
                            // <Link<Route> classes="nav-link" to={Route::Home}>
                            //     {"Forge a Link"}
                            // </Link<Route>>
                            <a class="nav-link" href="/#hero">
                                { "Forge a Link" }
                            </a>
                        </li>
                    </ul>
                    <span onclick={on_link_click.clone()}>
                        // <Link<Route>
                        //     classes="btn btn-brand rounded-pill ms-lg-3 d-lg-inline d-none"
                        //     to={Route::Forge}>
                        //     {"Forge a Link"}
                        // </Link<Route>>
                    <a class="btn btn-brand rounded-pill ms-lg-3 d-lg-inline d-none" href="/#hero"  onclick={on_link_click.clone()}>{ "Forge a Link" }</a>
                    </span>
                </div>
            </div>
        </nav>
    }
}