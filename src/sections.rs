use strum::IntoEnumIterator;

#[non_exhaustive]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::Display, strum_macros::EnumIter,
)]
pub enum SectionGroup {
    #[strum(serialize = "Special Classes (this year only!)")]
    Special,

    #[strum(serialize = "Any Picture")]
    AnyPicture,

    #[strum(serialize = "Needlecraft")]
    Needlecraft,

    #[strum(serialize = "Models")]
    Models,

    #[strum(serialize = "Miscellaneous")]
    Misc,
}

#[non_exhaustive]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum Section {
    #[strum(serialize = "None (or I don't know)")]
    None,

    #[strum(serialize = "{0}")]
    SpecialClass(String),

    #[strum(serialize = "Photography")]
    Photography,

    #[strum(serialize = "Painting")]
    Painting,

    #[strum(serialize = "Sketch/Drawing")]
    SketchOrDrawing,

    #[strum(serialize = "Knitting")]
    Knitting,

    #[strum(serialize = "Crochet")]
    Crochet,

    #[strum(serialize = "Sewing")]
    Sewing,

    #[strum(serialize = "Embroidery")]
    Embroidery,

    #[strum(serialize = "Digital Art")]
    DigitalArt,

    #[strum(serialize = "Collage/Mixed Media")]
    CollageMixedMedia,

    #[strum(serialize = "Clay/Plasticine Model")]
    ClayPlasticineModel,

    #[strum(serialize = "\"Junk\" Model")]
    JunkModel,

    #[strum(serialize = "Painted Pebble")]
    PaintedPebble,

    #[strum(serialize = "Decorated Cupcakes")]
    DecoratedCupcakes,

    #[strum(serialize = "Other Craft")]
    OtherCraft,
}

impl Section {
    pub fn parse_assuming_special_if_no_match(s: &str) -> Section {
        if let Ok(section) = s.parse() {
            section
        } else {
            Section::SpecialClass(s.to_owned())
        }
    }

    pub fn all_sections_with_specials_from_file() -> impl Iterator<Item = Section> {
        Self::all_sections_with_specials_from_file_content(include_str!("../res/special-class.txt"))
    }

    fn all_sections_with_specials_from_file_content(
        file_content: &str,
    ) -> impl Iterator<Item = Section> {
        let special = file_content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        Self::all_sections_with_specials(special)
    }

    fn all_sections_with_specials(
        special_classes: Vec<impl Into<String>>,
    ) -> impl Iterator<Item = Section> {
        let special = special_classes
            .into_iter()
            .map(|s| Section::SpecialClass(s.into()));

        let non_special =
            Section::iter().filter(|section| !matches!(section, Section::SpecialClass(_)));

        special.chain(non_special)
    }

    pub fn as_input(&self) -> Option<String> {
        match self {
            Self::None => None,
            others => Some(others.to_string()),
        }
    }

    pub fn group(&self) -> Option<SectionGroup> {
        match self {
            Section::None => None,

            Section::SpecialClass(_) => Some(SectionGroup::Special),

            Section::Painting
            | Section::SketchOrDrawing
            | Section::DigitalArt
            | Section::CollageMixedMedia => Some(SectionGroup::AnyPicture),

            Section::Knitting | Section::Crochet | Section::Sewing | Section::Embroidery => {
                Some(SectionGroup::Needlecraft)
            }

            Section::ClayPlasticineModel | Section::JunkModel => Some(SectionGroup::Models),

            Section::Photography
            | Section::PaintedPebble
            | Section::DecoratedCupcakes
            | Section::OtherCraft => Some(SectionGroup::Misc),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_assuming_special_if_no_match_for_special() {
        let section = Section::SpecialClass("Everything Elephant".to_owned());

        let serialised = section.to_string();
        assert_eq!("Everything Elephant", serialised);

        let restored = Section::parse_assuming_special_if_no_match(&serialised);
        assert_eq!(section, restored);
    }

    #[test]
    fn parse_assuming_special_if_no_match_for_non_special() {
        let section = Section::CollageMixedMedia;

        let serialised = section.to_string();
        assert_eq!("Collage/Mixed Media", serialised);

        let restored = Section::parse_assuming_special_if_no_match(&serialised);
        assert_eq!(section, restored);
    }

    #[test]
    fn every_section_including_specials_from_real_specials_file_serialises_uniquely() {
        let sections = Section::all_sections_with_specials_from_file().collect::<Vec<_>>();

        let unique_serialised: HashSet<_> = sections.iter().map(|s| s.to_string()).collect();
        let total_unique = unique_serialised.len();

        let total_non_unique = sections.len();
        assert_eq!(total_unique, total_non_unique);
    }

    #[test]
    fn every_section_including_specials_serialise_ignoring_whitespace_in_emulated_specials_file() {
        let sections = Section::all_sections_with_specials_from_file_content(
            "     Special 1
    2nd Special / + !

            THIRD SPECIAL      ",
        )
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let expected = vec![
            "Special 1",
            "2nd Special / + !",
            "THIRD SPECIAL",
            "None (or I don't know)",
            "Photography",
            "Painting",
            "Sketch/Drawing",
            "Knitting",
            "Crochet",
            "Sewing",
            "Embroidery",
            "Digital Art",
            "Collage/Mixed Media",
            "Clay/Plasticine Model",
            "\"Junk\" Model",
            "Painted Pebble",
            "Decorated Cupcakes",
            "Other Craft",
        ];

        assert_eq!(expected, sections);
    }
}
