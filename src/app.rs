use crate::{
    render::{TypstInputs, render_to_bytes},
    sections::{Section, SectionGroup},
};

use base64::{Engine as _, engine::general_purpose};
use leptos::prelude::*;
use leptos_use::signal_debounced;
use std::{collections::HashMap, iter};
use strum::IntoEnumIterator as _;

const DEBOUNCE_MS: f64 = 400.;
const PDF_FILENAME: &str = "prefilled-entry-card.pdf";

fn some_if_not_blank(signal_in: Signal<String>) -> Option<String> {
    Some(signal_in.get()).filter(|s| !s.trim().is_empty())
}

fn sections_iter_to_view(iter: Vec<Section>) -> impl IntoView {
    iter.iter().map(
        |section| view! { <option value={section.to_string()}>{section.display_name()}</option>},
    )
    .collect::<Vec<_>>()
}

fn section_options() -> impl IntoView {
    let mut grouped_sections: HashMap<_, Vec<_>> =
        Section::iter().fold(HashMap::new(), |mut map, section| {
            map.entry(section.group()).or_default().push(section);
            map
        });

    let optgroup_gen = move |group: SectionGroup, sections: Vec<Section>| {
        view! {
            <optgroup label=group.to_string()>
                { sections_iter_to_view(sections) }
            </optgroup>
        }
    };

    let group_gen = move |group_opt: Option<SectionGroup>, sections: Vec<Section>| match group_opt {
        Some(group) => optgroup_gen(group, sections).into_any(),
        None => sections_iter_to_view(sections).into_any(),
    };

    view! {
        {
            SectionGroup::iter()
                .map(Some)
                .chain(iter::once(None))
                .map(|group_opt| {
                    let sections = grouped_sections.remove(&group_opt).expect("group_opt must exist");
                    group_gen(group_opt, sections)
                })
                .collect_view()
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (contact_details, set_contact_details) = signal(String::new());
    let (entrants_name, set_entrants_name) = signal(String::new());
    let (entrants_age, set_entrants_age) = signal(String::new());
    let (section, set_section) = signal(Section::None);

    let debounced_contact_details: Signal<String> = signal_debounced(contact_details, DEBOUNCE_MS);
    let debounced_entrants_name: Signal<String> = signal_debounced(entrants_name, DEBOUNCE_MS);
    let debounced_entrants_age: Signal<String> = signal_debounced(entrants_age, DEBOUNCE_MS);

    let base64_pdf = Memo::new(move |_| {
        let contact_details = some_if_not_blank(debounced_contact_details);
        let entrants_name = some_if_not_blank(debounced_entrants_name);
        let entrants_age = some_if_not_blank(debounced_entrants_age);
        let section = section.get().as_input();
        let raw_pdf = render_to_bytes(TypstInputs {
            title: PDF_FILENAME.to_string(),
            contact_details,
            entrants_name,
            section,
            entrants_age,
        });
        general_purpose::STANDARD.encode(&raw_pdf)
    });
    let embed_pdf_src = move || format!("data:application/pdf;base64,{}", base64_pdf.get());

    view! {
        <div id="outer-container">
            <PrivacyBanner />
            <div id="inputs-container">
                <label for="contact-details">"Contact Details"</label>
                <input
                    id="contact-details"
                    type="text"
                    bind:value=(contact_details, set_contact_details)
                />

                <label for="entrant-name">"Entrant's Name"</label>
                <input
                    id="entrant-name"
                    type="text"
                    bind:value=(entrants_name, set_entrants_name)
                />

                <label for="entrant-age">"Entrant's Age"</label>
                <input
                    id="entrant-age"
                    type="number"
                    min=4
                    max=16
                    bind:value=(entrants_age, set_entrants_age)
                />

                <label for="section">"Section"</label>
                <select
                    name="section"
                    id="section"
                    on:change:target=move |ev| {
                        set_section.set(ev.target().value().parse().expect("options auto generated from the enum so must be valid unless tampered"));
                    }
                    prop:value=move || section.get().to_string()
                >
                    { section_options() }
                </select>

                <a href=embed_pdf_src download=PDF_FILENAME target="_blank">
                    <button type="button">"Save Pre-filled Entry Card PDF"</button>
                </a>
            </div>
            <embed
                id="embed-pdf"
                src=embed_pdf_src
                type="application/pdf"
                title="Pre-filled Entry Card Preview"
            />
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
                    "This tool does not transmit any data anywhere. All processing to generate the pre-filled pdf happens on your own device, and all data is lost when the page is closed so be sure to save your pre-filled pdf before leaving! This tool is "
                    <a href="https://github.com/MikeCroall/craft-show-entry-cards" target="_blank">
                        "open source"
                    </a>
                    "!"
                </div>
                <button type="button" on:click=move |_| set_show.set(false)>
                    Dismiss
                </button>
            </div>
        </Show>
    }
}
