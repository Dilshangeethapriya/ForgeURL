use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::components::{navbar::Navbar, footer::Footer, recent_links_item::RecentLinkItem};
use shared::types::{AnalyticsResponse, GetUrlResponse};
use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let analytics: UseStateHandle<Option<AnalyticsResponse>> = use_state(|| None);
    let error: UseStateHandle<Option<String>> = use_state(|| None);
    let loading: UseStateHandle<bool> = use_state(|| true);

    let search_query:   UseStateHandle<String>              = use_state(|| String::new());
    let search_results: UseStateHandle<Option<Vec<GetUrlResponse>>> = use_state(|| None);
    let search_loading: UseStateHandle<bool>                = use_state(|| false);
     {
        let analytics = analytics.clone();
        let error     = error.clone();
        let loading   = loading.clone();
 
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get("http://localhost:7878/analytics")
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        match resp.json::<AnalyticsResponse>().await {
                            Ok(data) => {
                                analytics.set(Some(data));
                                error.set(None);
                            }
                            Err(e) => error.set(Some(format!("Parse error: {e}"))),
                        }
                    }
                    Ok(resp) => {
                        error.set(Some(format!("Server error: {}", resp.status())));
                    }
                    Err(e) => {
                        error.set(Some(format!("Network error: {e}")));
                    }
                }
                loading.set(false);
            });
 
            || () // cleanup (none needed)
        });
    }


  
      let on_search = {
      let search_query   = search_query.clone();
      let search_results = search_results.clone();
      let search_loading = search_loading.clone();
      Callback::from(move |e: InputEvent| {
          let target: HtmlInputElement = e.target_unchecked_into();
          let query = target.value();
          search_query.set(query.clone());
          // clear results when input is empty
          if query.trim().is_empty() {
              search_results.set(None);
              return;
          }
          let search_results = search_results.clone();
          let search_loading = search_loading.clone();
          search_loading.set(true);
          // debounce — wait 400ms before firing the request
          Timeout::new(800, move || {
              let search_results = search_results.clone();
              let search_loading = search_loading.clone();
              spawn_local(async move {
                  let url = format!(
                      "http://localhost:7878/search?search_string={}",
                      query.trim()
                  );
                  match Request::get(&url).send().await {
                      Ok(resp) if resp.ok() => {
                          match resp.json::<Vec<GetUrlResponse>>().await {
                              Ok(data) => search_results.set(Some(data)),
                              Err(_e)   => search_results.set(Some(vec![])), // silently clear on parse error
                          }
                      }
                      _ => search_results.set(Some(vec![])),
                  }
                  search_loading.set(false);
              });
          })
          .forget();
      })
  };

  // switch search and popular links
  let table_rows = move |data: &AnalyticsResponse| {
      if let Some(ref results) = *search_results {
          results.clone()
      } else {
          data.popular_links.clone()
      }
  };


    html! {
        <>
        <Navbar />
         if *loading {
                    <>
                         <main class="d-flex align-items-center justify-content-center" style="min-height: 80vh;">
                             <div class="text-center">
                                 <div class="mb-4">
                                     <img src="assets/images/ForgeURL-hammer-tr.png" width="72" height="72"
                                         style="opacity: 0.6; animation: pulse 1.5s ease-in-out infinite;" />
                                 </div>
                                 <h5 class="fw-semibold mb-2">{ "Forging your analytics..." }</h5>
                                 <p class="text-muted">{ "Fetching data from the vault." }</p>
                                 <div class="spinner-border spinner-border-sm text-primary mt-2" role="status">
                                     <span class="visually-hidden">{ "Loading..." }</span>
                                 </div>
                             </div>
                         </main>
                     </>
        } else if let Some(err) = (*error).clone() {
          <>
            <main class="d-flex align-items-center justify-content-center" style="min-height: 80vh;">
                <div class="text-center">
                    <img src="assets/images/broken-link.png" width="72" height="72"
                        style="opacity: 0.5;" class="mb-4" />
                    <h5 class="fw-semibold mb-2">{ "Something went wrong" }</h5>
                    <p class="text-muted mb-3">{ "Could not connect to the ForgeURL backend." }</p>
                    <div class="alert alert-danger rounded-3 d-inline-block px-4 py-2 small">
                        { err }
                    </div>
                    <div class="mt-4">
                        <a href="/dashboard" class="btn btn-brand rounded-pill px-4">
                            { "Try Again" }
                        </a>
                    </div>
                </div>
            </main>
          </>
                }
         else if let Some(data) = (*analytics).clone() {
            <main>
           <section class="dashboard py-5 bg-light" id="dashboard">
             <div class="container">
               <div class="mb-4">
                 <h2 class="fw-bold">{ "Dashboard" }</h2>
               </div>
     
               // Count cards 
               <div class="row g-4 mb-5">
                 <div class="col-md-4">
                   <div class="counts-card bg-white p-4 rounded-4 h-100">
                     <h6 class="text-muted mb-2">{ "Total Links" }</h6>
                     <h3 class="fw-bold mb-0">{ data.total_links }</h3>
                   </div>
                 </div>
     
                 <div class="col-md-4">
                   <div class="counts-card bg-white p-4 rounded-4 h-100">
                     <h6 class="text-muted mb-2">{ "Total Clicks" }</h6>
                     <h3 class="fw-bold mb-0">{ data.total_clicks }</h3>
                   </div>
                 </div>
     
                  if let Some(top) = data.most_popular_link.clone() {
                 <div class="col-md-4 over">
                   <div class="counts-card bg-white p-4 rounded-4 h-100">
                     <h6 class="text-muted mb-2">{ "Most Clicked Link" }</h6>
                     <p class="fw-bold mb-0 fs-">{ top.short_url.trim_start_matches("http://").trim_start_matches("https://") }</p>
                   </div>
                 </div>
                   }
               </div>
              
               //Popular Links / search results
               <div class="bg-white rounded-4 shadow-lg p-4">
                 <div class="d-flex justify-content-between align-items-center mb-3">
                    <h3 class="fw-bold mb-4 text-center w-100">
                    if (*search_query).is_empty() {
                      { "Popular Links" }
                    } else {
                      {"Search Results"}
                    }
                  </h3>
                 </div>
     
                 <div class="row mb-4 justify-content-center">
                   <div class="col-md-6">
                     <input
                       type="text"
                       class="form-control rounded-pill shadow-sm"
                       placeholder="Search links..."
                       id="searchLinks"
                       oninput={on_search}
                        />
                   </div>
                 </div>
     
                 <div class="table-responsive">
                   <table class="table align-middle mb-0">
                     <thead class="table-light">
                       <tr>
                         <th>{ "Original URL" }</th>
                         <th>{ "Short URL" }</th>
                         <th>{ "Clicks" }</th>
                         <th>{ "Created" }</th>
                       </tr>
                     </thead>
                     <tbody>
                        if *search_loading {
                          <tr>
                            <td colspan="4" class="text-center text-muted py-3">
                              <span class="spinner-border spinner-border-sm me-2" role="status"></span>
                              { "Searching..." }
                            </td>
                          </tr>
                        } else {
                          { for table_rows(&data).iter().map(|link| html! {
                              <tr>
                                <td class="text-truncate" style="max-width: 280px">
                                  { &link.original_url }
                                </td>
                                <td>
                                  <a href={ link.short_url.clone() }  target="_blank">{ &link.short_url }</a>
                                </td>
                                <td>{ link.click_count }</td>
                                <td class="text-muted">{ &link.created_at }</td>
                              </tr>
                          })}
                          if table_rows(&data).is_empty() {
                            <tr>
                              <td colspan="4" class="text-center text-muted py-3">
                                { "No links found." }
                              </td>
                            </tr>
                          }
                        }
                    </tbody>
                   </table>
                 </div>
               </div>
             </div>
           </section>
           // Recently created URL's 
           <section class="recent-activity py-5 bg-light">
             <div class="container">
               <h3 class="fw-bold mb-4 text-center">{ "Recently Created Links" }</h3>
     
               <div class="list-group">
                { for data.recent_links.iter().map(|link| html! {
                    <RecentLinkItem link={link.clone()} />
                 })}
               </div>
             </div>
           </section>
         </main>
         }
         else {
                 <>
                    <main class="d-flex align-items-center justify-content-center" style="min-height: 80vh;">
                        <div class="text-center">
                            <img src="assets/images/broken-link.png" width="72" height="72"
                                style="opacity: 0.5;" class="mb-4" />
                            <h5 class="fw-semibold mb-2">{ "No data yet" }</h5>
                            <p class="text-muted mb-4">{ "Forge your first link to see analytics here." }</p>
                            <a href="/" class="btn btn-brand rounded-pill px-4">
                                { "Forge a Link" }
                            </a>
                        </div>
                    </main>
                 </>
                  
              }
        <Footer />
        </>
    }
}