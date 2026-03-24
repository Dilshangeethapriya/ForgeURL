use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{navbar::Navbar, footer::Footer};
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <>
            <Navbar />
            <main>
                <section class="not-found d-flex align-items-center justify-content-center" id="not-found">
                    <div class="container not-found-container p-5 my-5 mx-sm-0 m-md-5">
                        <div class="row my-5 w-100 justify-content-center">
                            <div class="col-lg-7 col-sm-12 d-flex flex-column align-items-center gap-3 text-center">

                                // Broken chain icon
                                <div class="not-found-icon mb-2">
                                    <img
                                        src="assets/images/broken-link.png"
                                        alt="Broken link"
                                        width="120"
                                        height="120" />
                                </div>

                                // 404 code
                                <h1 class="display-2 fw-semibold text-primary">
                                    { "404" }
                                </h1>

                                <h1 class="fw-semibold">
                                    { "Page Not Found" }
                                </h1>

                                <h5 class="text-muted">
                                    { "The page you're looking for doesn't exist, has expired, or may have been deleted." }
                                </h5>

                                // Action buttons
                                <div class="d-flex flex-column flex-sm-row gap-2 w-100 justify-content-center mt-2">
                                    <button
                                        onclick={go_home}
                                        class="btn btn-brand rounded-pill px-4">
                                        { "Back to ForgeURL" }
                                    </button>
                                </div>

                            </div>
                        </div>

                    </div>
                </section>
            </main>
            <Footer />
        </>
    }
}