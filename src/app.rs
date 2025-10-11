use crate::render::{render_to_bytes, TypstInputs};

use base64::{engine::general_purpose, Engine as _};
use leptos::prelude::*;

fn some_if_not_blank(signal_in: ReadSignal<String>) -> Option<String> {
    Some(signal_in.get()).filter(|s| !s.trim().is_empty())
}

#[component]
pub fn App() -> impl IntoView {
    let (contact_details, set_contact_details) = signal("".to_string());
    let (entrants_name, set_entrants_name) = signal("".to_string());
    let (entrants_age, set_entrants_age) = signal("".to_string());

    let pdf_base64 = Memo::new(move |_| {
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
    let embed_pdf_src = move || format!("data:application/pdf;base64,{}", pdf_base64.get());

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
        <button on:click=move |_| {}>"Save Entry Card PDF"</button>
        <br />
        <embed src=embed_pdf_src />
    }
}
