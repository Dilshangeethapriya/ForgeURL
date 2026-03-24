use yew::prelude::*;
use crate::components::{navbar::Navbar, footer::Footer};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <Navbar />
            <main>
                <section class="py-5">
                    <div class="container">
                        <div class="row justify-content-center mb-5">
                            <div class="col-lg-8 text-center">
                                <h2 class="fw-bold mb-3">{ "About ForgeURL" }</h2>
                                <p class="text-muted">
                                    { "ForgeURL is a lightweight URL shortener built as a " }
                                    <strong>{ "technical demonstration" }</strong>
                                    { " to explore modern Rust-based backend development and full-stack system design." }
                                </p>
                            </div>
                        </div>

                        <div class="row g-4">
                            <div class="col-md-6">
                                <div class="p-4 rounded-4 bg-white shadow-sm h-100">
                                    <h5 class="fw-semibold mb-3">{ "What is ForgeURL?" }</h5>
                                    <p class="text-muted mb-0">
                                        { "ForgeURL is a " }
                                        <strong>{ "demo / prototype project" }</strong>
                                        { " designed to showcase how a fast, secure URL-shortening service can be built using the Rust programming language." }
                                        <br /><br />
                                        { "Rather than focusing on marketing or production features, the goal of this project is to highlight " }
                                        <strong>{ "clean architecture" }</strong>
                                        { ", " }
                                        <strong>{ "performance-aware design" }</strong>
                                        { ", and " }
                                        <strong>{ "safe backend development" }</strong>
                                        { "." }
                                    </p>
                                </div>
                            </div>

                            <div class="col-md-6">
                                <div class="p-4 rounded-4 bg-white shadow-sm h-100">
                                    <h5 class="fw-semibold mb-3">{ "Technical Focus" }</h5>
                                    <ul class="text-muted mb-0">
                                        <li>
                                            { "High-performance HTTP APIs using " }
                                            <strong>{ "Axum" }</strong>
                                        </li>
                                        <li>
                                            { "Strongly typed request/response handling with " }
                                            <strong>{ "Serde" }</strong>
                                        </li>
                                        <li>
                                            { "Lightweight persistence using " }
                                            <strong>{ "SQLite" }</strong>
                                        </li>
                                        <li>
                                            { "WebAssembly-based frontend with " }
                                            <strong>{ "Yew" }</strong>
                                        </li>
                                        <li>{ "Clear separation between backend logic and UI" }</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="col-md-6">
                                <div class="p-4 rounded-4 bg-white shadow-sm h-100">
                                    <h5 class="fw-semibold mb-3">{ "Architecture Overview" }</h5>
                                    <p class="text-muted mb-0">
                                        { "The backend is built using Rust with a focus on " }
                                        <strong>{ "memory safety" }</strong>
                                        { ", " }
                                        <strong>{ "low latency" }</strong>
                                        { ", and " }
                                        <strong>{ "predictable performance" }</strong>
                                        { ". " }
                                        <br /><br />
                                        { "The frontend is written in " }
                                        <strong>{ "Yew" }</strong>
                                        { " and compiled to WebAssembly, allowing Rust to be used across the entire stack while keeping the UI responsive and maintainable." }
                                    </p>
                                </div>
                            </div>

                            <div class="col-md-6">
                                <div class="p-4 rounded-4 bg-white shadow-sm h-100">
                                    <h5 class="fw-semibold mb-3">{ "Project Purpose" }</h5>
                                    <p class="text-muted mb-0">
                                        { "This project exists primarily as a " }
                                        <strong>{ "learning and showcase platform" }</strong>
                                        { ". It demonstrates backend API design, data modeling, and system thinking using Rust." }
                                        <br /><br />
                                        { "While ForgeURL may be expanded in the future with features such as authentication or advanced analytics, it currently serves as a focused prototype for exploring core engineering concepts." }
                                    </p>
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