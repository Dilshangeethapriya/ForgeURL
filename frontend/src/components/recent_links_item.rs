use shared::types::GetUrlResponse;
use yew::use_state;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RecentLinkItemProps {
    pub link: GetUrlResponse,
}

#[function_component(RecentLinkItem)]
pub fn recent_link_item(props: &RecentLinkItemProps) -> Html {
    let expanded = use_state(|| false);
    let link = &props.link;

    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_: MouseEvent| expanded.set(!*expanded))
    };

    html! {
        <div class="list-group-item list-group-item-action d-flex flex-column flex-md-row justify-content-between align-items-start rounded-4 mb-3 shadow-sm bg-white">
            <div class="d-flex flex-column flex-md-row align-items-start align-items-md-center gap-3">
                <span class="badge bg-primary rounded-pill">{ "Created" }</span>
                <div>
                    <p class="mb-1 fw-semibold">
                        { " Short URL: " }
                        <a href={ link.short_url.clone() }  target="_blank" class="text-decoration-none">
                            { &link.short_url }
                        </a>
                    </p>
                    <p class="mb-0 text-muted text-break">
                        if *expanded {
                            { &link.original_url }
                            { " " }
                            <span
                                onclick={toggle}
                                style="cursor:pointer; font-size: 0.8rem;"
                                class="text-primary">
                                { "see less" }
                            </span>
                        } else {
                            // Show first 60 chars then "see more"
                            { format!("{}...", &link.original_url[..60.min(link.original_url.len())]) }
                            { " " }
                            <span
                                onclick={toggle}
                                style="cursor:pointer; font-size: 0.8rem;"
                                class="text-primary">
                                { "see more" }
                            </span>
                        }
                    </p>
                </div>
            </div>
            <small class="text-muted mt-2 mt-md-0">{ &link.created_at }</small>
        </div>
    }
}