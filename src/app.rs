use crate::render::{TypstInputs, render_to_bytes};

use base64::{Engine as _, engine::general_purpose};
use leptos::prelude::*;

fn some_if_not_blank(signal_in: ReadSignal<String>) -> Option<String> {
    Some(signal_in.get()).filter(|s| !s.trim().is_empty())
}

#[component]
pub fn App() -> impl IntoView {
    let (contact_details, set_contact_details) = signal("".to_string());
    let (entrants_name, set_entrants_name) = signal("".to_string());
    let (entrants_age, set_entrants_age) = signal("".to_string());

    let base64_pdf = Memo::new(move |_| {
        let contact_details = some_if_not_blank(contact_details);
        let entrants_name = some_if_not_blank(entrants_name);
        let entrants_age = some_if_not_blank(entrants_age);
        let raw_pdf = render_to_bytes(TypstInputs {
            contact_details,
            entrants_name,
            entrants_age,
        });
        general_purpose::STANDARD.encode(&raw_pdf)
    });
    let embed_pdf_src =
        Memo::new(move |_| format!("data:application/pdf;base64,{}", base64_pdf.get()));

    view! {
        <div id="outer-container">
            <PrivacyBanner />
            <div id="inputs-container">
                <label>"Contact Details"</label>
                <input type="text" bind:value=(contact_details, set_contact_details) />
                <label>"Entrant's Name"</label>
                <input type="text" bind:value=(entrants_name, set_entrants_name) />
                <label>"Entrant's Age"</label>
                <input type="number" bind:value=(entrants_age, set_entrants_age) />
                <a href=embed_pdf_src download="prefilled-entry-card.pdf" target="_blank">
                    <button type="button">"Save Pre-filled Entry Card PDF"</button>
                </a>
            </div>
            <embed id="embed-pdf" src=embed_pdf_src title="Pre-filled Entry Card Preview" />
        </div>
    }
}

#[component]
fn PrivacyBanner() -> impl IntoView {
    let (show, set_show) = signal(true);

    view! {
        <Show when=move || show.get()>
            <div id="privacy-banner" class:hidden=move || show.get()>
                <div>
                    <span class="emphasize space-after">"Privacy Notice"</span>
                    "This tool does not transmit any data anywhere. All processing to generate the pre-filled pdf happens on your own device, and all data is lost when the page is closed so be sure to save your pre-filled pdf before leaving!"
                </div>
                <button type="button" on:click=move |_| set_show.set(false)>
                    Dismiss
                </button>
            </div>
        </Show>
    }
}
