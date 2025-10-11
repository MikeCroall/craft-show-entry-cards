use derive_typst_intoval::{IntoDict, IntoValue};
use typst::foundations::IntoValue;
use typst_as_lib::TypstEngine;

static TEMPLATE_FILE: &str = include_str!("../typst/entry-card.typ");
static FONT: &[u8] = include_bytes!("../typst/fonts/LiberationSans-Regular.ttf");
static FONT_BOLD: &[u8] = include_bytes!("../typst/fonts/LiberationSans-Bold.ttf");
static FONT_HANDWRITE: &[u8] = include_bytes!("../typst/fonts/PatrickHand-Regular.ttf");
static SCISSORS_SVG: &[u8] = include_bytes!("../typst/icons/scissors.svg");
static FOLD_SVG: &[u8] = include_bytes!("../typst/icons/fold.svg");

#[derive(Debug, Default, Clone, IntoValue, IntoDict)]
pub struct TypstInputs {
    pub contact_details: Option<String>,
    pub entrants_name: Option<String>,
    pub entrants_age: Option<String>,
}

pub fn render_to_bytes(inputs: TypstInputs) -> Vec<u8> {
    let template = TypstEngine::builder()
        .main_file(TEMPLATE_FILE)
        .fonts([FONT, FONT_BOLD, FONT_HANDWRITE])
        .with_static_file_resolver([
            ("icons/scissors.svg", SCISSORS_SVG),
            ("icons/fold.svg", FOLD_SVG),
        ])
        .build();

    let doc = template
        .compile_with_input(inputs.into_dict())
        .output
        .expect("typst compile error");

    let options = Default::default();

    typst_pdf::pdf(&doc, &options).expect("pdf generation error")
}

#[cfg(test)]
mod tests {
    use crate::render::render_to_bytes;

    #[test]
    fn typst_compiles() {
        let _ = render_to_bytes(Default::default());
    }

    #[test]
    fn typst_compiles_with_inputs() {
        let _ = render_to_bytes(super::TypstInputs {
            contact_details: Some("Example Contact Details".to_string()),
            entrants_name: None,
            entrants_age: Some("12".to_string()),
        });
    }
}
