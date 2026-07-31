use typst_bake::{IntoDict, IntoValue, document};

#[derive(Debug, Default, Clone, IntoValue, IntoDict)]
pub struct TypstInputs {
    pub title: String,
    pub contact_details: Option<String>,
    pub entrants_name: Option<String>,
    pub entrants_age: Option<String>,
}

pub fn render_to_bytes(inputs: TypstInputs) -> Vec<u8> {
    document!("entry-card.typ")
        .with_inputs(inputs)
        .to_pdf()
        .expect("typst compile error")
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
            title: "test-file.pdf".to_string(),
            contact_details: Some("Example Contact Details".to_string()),
            entrants_name: None,
            entrants_age: Some("12".to_string()),
        });
    }
}
