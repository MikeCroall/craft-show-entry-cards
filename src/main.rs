use derive_typst_intoval::{IntoDict, IntoValue};
use typst_as_lib::TypstEngine;

static TEMPLATE_FILE: &str = include_str!("../typst/entry-card.typ");
static FONT: &[u8] =
    include_bytes!("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf");
static FONT_BOLD: &[u8] =
    include_bytes!("/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf");
static OUT_FILE: &str = "./typst/entry-card-rust-out.pdf";

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct TypstInputs {}

fn main() {
    let inputs = TypstInputs {};

    let template = TypstEngine::builder()
        .main_file(TEMPLATE_FILE)
        .fonts([FONT, FONT_BOLD])
        .with_file_system_resolver("./typst")
        .build();

    let doc = template
        .compile_with_input(inputs.into_dict())
        .output
        .expect("typst compile error");

    let options = Default::default();
    let pdf = typst_pdf::pdf(&doc, &options).expect("pdf generation error");

    std::fs::write(OUT_FILE, pdf).expect("pdf write error");

    println!("Wrote out to {OUT_FILE}");
}
