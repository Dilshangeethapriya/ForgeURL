use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
      <div class="f-top">
        <div class="container">
          <div class="row gy-5">
            //Logo 
            <div class="col-md-4 d-flex flex-column">
              <a href="#" class="mb-3">
                <img
                  src="assets/images/ForgeURL-tr.png"
                  alt="ForgeURL logo" />
              </a>

              <p class="footer-desc">
               { "A fast, secure URL shortener built with modern Rust and
                WebAssembly."}
              </p>

              <div class="social-links mt-3">
                <a href="https://github.com/Dilshangeethapriya"
                  ><i class="bi bi-github"></i
                ></a>
                <a
                  href="https://www.linkedin.com/in/dilshan-geethappriya-b30530200/"
                  ><i class="bi bi-linkedin"></i
                ></a>
                <a href="https://www.instagram.com/dilshangeethapriya/"
                  ><i class="bi bi-instagram"></i
                ></a>
                <a href="https://wa.me/+94773931851"
                  ><i class="bi bi-whatsapp"></i
                ></a>
              </div>
            </div>

            <div class="col-md-4 d-flex flex-column">
              <h6 class="footer-title">{ "Quick Links" }</h6>
              <div class="line"></div>

              <ul class="footer-links">
                <li><a href="index.html">{"Home"}</a></li>
                <li><a href="dashboard.html">{"Dashboard"}</a></li>
                <li><a href="apiDocs.html">{"API Docs"}</a></li>
                <li><a href="about.html">{"About"}</a></li>
                <li><a href="index.html#hero">{"Forge a Link"}</a></li>
              </ul>
            </div>

            <div class="col-md-4 d-flex flex-column">
              <h6 class="footer-title">{ "Tech Stack" }</h6>
              <div class="line"></div>

              <ul class="footer-tech">
                <li>{"Backend: Rust • Axum • Serde"}</li>
                <li>{"Frontend: Yew • WebAssembly"}</li>
                <li>{"Database: SQLite"}</li>
                <li>{"Styling: Bootstrap"}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <div class="f-bottom">
        <div class="container">
          <div class="row justify-content-between gy-3">
            <div class="col-auto">
              {"© 2025–2026 ForgeURL. All rights reserved."}
            </div>
            <div class="col-auto">
              {"Designed with care by "}
              <a href="https://dilshangeethappriya.netlify.app/"
                >{ "Dilshan Geethappriya" }</a
              >
            </div>
          </div>
        </div>
      </div>
    </footer>
    }
}