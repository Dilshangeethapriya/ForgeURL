use yew::prelude::*;
use crate::components::{navbar::Navbar, footer::Footer};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use shared::types::{ShortenRequest, GetUrlResponse};
use gloo_timers::callback::Timeout;
use crate::api::API_BASE;

#[function_component(Home)]
pub fn home() -> Html {

    let input_url   = use_state(|| String::new());
    let result      = use_state(|| None::<GetUrlResponse>);
    let error       = use_state(|| None::<String>);
    let loading     = use_state(|| false);
    let copied = use_state(|| false);

    // handling input change
    let on_input = {
        let input_url = input_url.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            input_url.set(target.value());
        })
    };


    let on_submit = {
        let input_url = input_url.clone();
        let result    = result.clone();
        let error     = error.clone();
        let loading   = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();            // stop page reload

            let url = (*input_url).clone();
            if url.trim().is_empty() {
                error.set(Some("Please enter a URL.".into()));
                return;
            }

            if !url.starts_with("http://") && !url.starts_with("https://") {
                error.set(Some("URL must start with http:// or https://".into()));
                return;
            }

            let result  = result.clone();
            let error   = error.clone();
            let loading = loading.clone();

            loading.set(true);
            result.set(None);
            error.set(None);

            spawn_local(async move {
                let body = ShortenRequest { original_url: url };

                match Request::post(&format!("{}/", API_BASE))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        match resp.json::<GetUrlResponse>().await {
                            Ok(data) => result.set(Some(data)),
                            Err(e)   => error.set(Some(format!("Parse error: {e}"))),
                        }
                    }
                    Ok(resp) => error.set(Some(format!("Server error: {}", resp.status()))),
                    Err(e)   => error.set(Some(format!("Network error: {e}"))),
                }
                loading.set(false);
            });
        })
    };

    html! {
      <>
         <Navbar />
         <main>
           // Hero 
           <section class="hero d-flex align-items-center justify-content-center" id="hero">
             <div class="container hero-container p-5 my-5 mx-sm-0 m-md-5">
               <div class="row my-5 w-100">
                 <div
                   class="col-lg-6 col-sm-12 mx-lg-0 mx-md-4 mx-sm-0 d-flex flex-column gap-3 text-center text-lg-start">
                   <h1 class="fw-semibold">{ "Forge short links. Fast." }</h1>
                   <h5>{ "A blazing-fast, secure URL shortener powered by Rust" }</h5>
                   <form class="w-100" onsubmit={on_submit}>
                     <input
                       type="text"
                       class="form-control mb-2 rounded-pill"
                       id="longUrl"
                       placeholder="Enter your long URL"
                       value={ (*input_url).clone() }
                       oninput={on_input}  
                       />
                     <button
                       type="submit"
                       class="btn btn-brand mb-2 w-100 rounded-pill"
                       disabled={*loading}>
                       if *loading {
                           <>
                             <span class="spinner-border spinner-border-sm me-2" role="status"></span>
                             { "Forging..." }
                           </>
                       } else {
                           { "Forge a Link" }
                       }
                     </button>
                   </form>
                    if let Some(err) = (*error).clone() {
                       <div class="alert alert-danger rounded-3 small py-2">{ err }</div>
                   }


                   if let Some(res) = (*result).clone() {
                       <div class="features-card rounded-4 p-3 text-start">
                           <p class="text-muted small mb-1">{ "Your forged link:" }</p>
                           <div class="d-flex align-items-center gap-2 mb-2">
                                <a
                                 href={res.short_url.clone()}
                                 target="_blank"
                                 class="fw-semibold text-primary text-decoration-none fs-5 text-break">
                                 { &res.short_url.trim_start_matches("http://").trim_start_matches("https://") }
                               </a>
                               // copy button
                               <button
                                   class="btn btn-sm rounded px-2 py-1 copy-btn"
                                   onclick={
                                       let url = res.short_url.clone();
                                       let copied = copied.clone();
                                       Callback::from(move |_: MouseEvent| {
                                           if let Some(window) = web_sys::window() {
                                               let _ = window.navigator().clipboard().write_text(&url);
                                           }
                               
                                           
                                           copied.set(true);
                                           let copied_reset = copied.clone();
                                           Timeout::new(2_000, move || {
                                               copied_reset.set(false);
                                           })
                                           .forget(); 
                                       })
                                   }>
                                   if *copied {
                                       <i class="bi bi-check-lg text-success"></i>
                                   } else {
                                       <i class="bi bi-copy"></i>
                                   }
                               </button>
                           </div>
                           <p class="text-muted small text-break mb-0 text-break">
                              <span class="text-primary">
                                <i class="bi bi-arrow-return-right"></i> 
                              </span>
                              { format!(" {}", &res.original_url) }
                           </p>
                       </div>
                   }

                 </div>
               </div>
             </div>
           </section>
           //Features 
           <section class="features py-5">
             <div class="container">
               <div class="row text-start g-4">
                 <div class="col-md-4 d-flex">
                   <div class="mx-1 features-card rounded-4 px-3 pt-2 pb-1 w-100">
                     <img
                       src="assets/images/rocket.svg"
                       alt="Rocket"
                       width="100"
                       height="100" />
                     <h5 class="fw-semibold h3">{ "Fast" }</h5>
                     <p class="text-muted">
                       { "Built with Rust for low-latency redirects." }
                     </p>
                   </div>
                 </div>
                 <div class="col-md-4 d-flex">
                   <div class="mx-1 features-card rounded-4 px-3 pt-2 pb-1 w-100">
                     <img
                       src="assets/images/padlock.svg"
                       alt="Rocket"
                       width="100"
                       height="100" />
                     <h5 class="fw-semibold h3">{ "Secure" }</h5>
                     <p class="text-muted">
                       { "Minimal attack surface and safe memory handling." }
                     </p>
                   </div>
                 </div>
                 <div class="col-md-4 d-flex">
                   <div class="mx-1 features-card rounded-4 px-3 pt-2 pb-1 w-100">
                     <img
                       src="assets/images/crab.svg"
                       alt="Rocket"
                       width="100"
                       height="100" />
                     <h5 class="fw-semibold h3">{ "Rust-powered" }</h5>
                     <p class="text-muted">
                       { "Axum + SQLite for modern backend architecture." }
                     </p>
                   </div>
                 </div>
               </div>
             </div>
           </section>
     
           // How it works
           <section class="how-it-works py-5">
             <div class="container">
               <h3 class="text-center fw-bold mb-5 pb-1 border-bottom border-2">
                 { "How it works" }
               </h3>
     
               <div class="row align-items-center mb-4">
                 <div class="col-md-2 text-center">
                   <span class="display-5 fw-bold text-primary">{ "01" }</span>
                 </div>
                 <div class="col-md-10">
                   <h6 class="fw-semibold">{ "Paste your URL" }</h6>
                   <p class="text-muted mb-0">
                     { "Enter the original link you want to shorten or track." }
                   </p>
                 </div>
               </div>
     
               <div class="row align-items-center mb-4">
                 <div class="col-md-2 text-center">
                   <span class="display-5 fw-bold text-primary">{ "02" }</span>
                 </div>
                 <div class="col-md-10">
                   <h6 class="fw-semibold">{ "Generate your link" }</h6>
                   <p class="text-muted mb-0">
                     { "Our system instantly creates a unique code." }
                   </p>
                 </div>
               </div>
     
               <div class="row align-items-center">
                 <div class="col-md-2 text-center">
                   <span class="display-5 fw-bold text-primary">{ "03" }</span>
                 </div>
                 <div class="col-md-10">
                   <h6 class="fw-semibold">{ "Share and track" }</h6>
                   <p class="text-muted mb-0">
                     { "Share your link and view analytics in real time." }
                   </p>
                 </div>
               </div>
             </div>
           </section>
         </main>
            <Footer />
      </>
        
    } 
} 