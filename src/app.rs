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
        <label>"Contact Details"</label>
        <input type="text" bind:value=(contact_details, set_contact_details) />
        <br />
        <label>"Entrant's Name"</label>
        <input type="text" bind:value=(entrants_name, set_entrants_name) />
        <br />
        <label>"Entrant's Age"</label>
        <input type="text" bind:value=(entrants_age, set_entrants_age) />
        <br />
        <a href=embed_pdf_src download="prefilled-entry-card.pdf" target="_blank">
            "Save Pre-filled Entry Card PDF"
        </a>
        <br />
        <embed
            src=embed_pdf_src
            title="Pre-filled Entry Card Preview"
            width="100%"
            height="800px"
        />
    }
}
