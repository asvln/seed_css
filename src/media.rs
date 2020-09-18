use super::units;

pub enum MediaType {
    All,
    Print,
    OnlyPrint,
    Screen,
    OnlyScreen,
    Speech,
    OnlySpeech,
}

fn match_media_type(media_type: MediaType) -> &'static str {
    match media_type {
        MediaType::All => "all",
        MediaType::Print => "print",
        MediaType::OnlyPrint => "only print",
        MediaType::Screen => "screen",
        MediaType::OnlyScreen => "only screen",
        MediaType::Speech => "speech",
        MediaType::OnlySpeech => "only speech",
    }
}

pub enum MediaFeature {
    AnyHover(Hover),
    AnyPointer(Pointer),
    AspectRatio((i16, i16)),
    Color(Range),
    ColorGamut(ColorGamut),
    ColorIndex(Range),
    DisplayMode(DisplayMode),
    ForcedColors(Active),
    Grid(bool),
    Height(Range),
    Hover(Hover),
    InvertedColors(Inverted),
    Monochrome(Range),
    Orientation(Orientation),
    OverflowBlock(OverflowBlock),
    OverflowInline(OverflowInline),
    Pointer(Pointer),
    PrefersColorScheme(ColorScheme),
    PrefersContrast(Contrast),
    PrefersReducedMotion(Reduce),
    PrefersReducedTransparency(Reduce),
    Resolution(units::Resolution),
    Scan(Scan),
    Scripting(Scripting),
    Update(Update),
    Width(Range),
}

pub enum Active {
    None,
    Active,
}

pub enum ColorGamut {
    Srgb,
    P3,
    Rec2020,
}

pub enum ColorScheme {
    Light,
    Dark,
}

pub enum Contrast {
    NoPreference,
    More,
    Less,
}

pub enum DisplayMode {
    Fullscreen,
    Standalone,
    MinimalUi,
    Browser,
}

pub enum Hover {
    None,
    Hover,
}

pub enum Inverted {
    None,
    Inverted,
}

pub enum Orientation {
    Portrait,
    Landscape,
}

pub enum OverflowBlock {
    None,
    Scroll,
    OptionalPaged,
    Paged,
}

pub enum OverflowInline {
    None,
    Scroll,
}

pub enum Pointer {
    None,
    Coarse,
    Fine,
}

pub enum Range {
    None,
    Min(units::Length),
    Max(units::Length),
}

pub enum Reduce {
    NoPreference,
    Reduce,
}

pub enum Scan {
    Interlace,
    Progressive,
}

pub enum Scripting {
    None,
    InitialOnly,
    Enabled,
}

pub enum Update {
    None,
    Slow,
    Fast,
}

fn match_media_query(media_query: MediaFeature) -> &'static str {
    use MediaFeature::*;
    match media_query {
        AnyHover(v) => match v {
            self::Hover::None => "(any-hover: none)",
            self::Hover::Hover => "(any-hover: hover)",
        },
        Hover(v) => match v {
            self::Hover::None => "(hover: none)",
            self::Hover::Hover => "(hover: hover)",
        },
        _ => "",
    }
}

#[macro_export]
macro_rules! at_media {
    ($media_type:ident $($operator:literal $media_query:ident)* , $css:literal ) => {
        print!("@media ");
        print!("{} ", match_media_type(crate::media::MediaType::$media_type));
        $(print!("{} {} ", $operator, match_media_query(crate::media::MediaFeature::$media_query));)*
        print!("{{ {} }}", $css);
    }
}
