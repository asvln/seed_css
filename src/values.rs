use derive_more::Display;

#[derive(Display, Clone, Debug)]
#[display(fmt = "color: {};")]
pub enum CssColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64),
    #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}
