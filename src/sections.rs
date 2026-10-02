#[non_exhaustive]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::Display, strum_macros::EnumIter,
)]
pub enum SectionGroup {
    #[strum(serialize = "Special Class (this year only!)")]
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
    Debug, Clone, Copy, strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter,
)]
pub enum Section {
    #[strum(serialize = "None (or I don't know)")]
    None,

    /// A special case that is serialized as-is, but the custom [`Self::display_name`] function will read the name to display from a local resource file. Therefore, there is no strum `serialize` annotation.
    SpecialClass,

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
    pub fn display_name(self) -> String {
        match self {
            Self::SpecialClass => include_str!("../res/special-class.txt").trim().to_owned(),
            _ => self.to_string(),
        }
    }

    pub fn as_input(self) -> Option<String> {
        match self {
            Self::None => None,
            others => Some(others.display_name()),
        }
    }

    pub fn group(self) -> Option<SectionGroup> {
        match self {
            Section::SpecialClass => Some(SectionGroup::Special),

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

            Section::None => None,
        }
    }
}
