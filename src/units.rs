use crate::values::CssColor;
use ordered_float::NotNan;

#[derive(Clone, Debug)]
pub struct UnitValue {
    pub unit: Unit,
    pub value: NotNan<f64>,
}

impl std::fmt::Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit {
            // Relative
            Unit::Length(Length::Em) => write!(f, "{}em", self.value),
            Unit::Length(Length::Ex) => write!(f, "{}ex", self.value),
            Unit::Length(Length::Cap) => write!(f, "{}cap", self.value),
            Unit::Length(Length::Ch) => write!(f, "{}ch", self.value),
            Unit::Length(Length::Ic) => write!(f, "{}ic", self.value),
            Unit::Length(Length::Rem) => write!(f, "{}rem", self.value),
            Unit::Length(Length::Lh) => write!(f, "{}lh", self.value),
            Unit::Length(Length::Rlh) => write!(f, "{}rlh", self.value),
            Unit::Length(Length::Vw) => write!(f, "{}vw", self.value),
            Unit::Length(Length::Vh) => write!(f, "{}vh", self.value),
            Unit::Length(Length::Vi) => write!(f, "{}vi", self.value),
            Unit::Length(Length::Vb) => write!(f, "{}vb", self.value),
            Unit::Length(Length::Vmin) => write!(f, "{}vmin", self.value),
            Unit::Length(Length::Vmax) => write!(f, "{}vmax", self.value),
            Unit::Length(Length::Percent) => write!(f, "{}%", self.value),
            // Absolute
            Unit::Length(Length::Cm) => write!(f, "{}cm", self.value),
            Unit::Length(Length::Mm) => write!(f, "{}mm", self.value),
            Unit::Length(Length::Q) => write!(f, "{}Q", self.value),
            Unit::Length(Length::In) => write!(f, "{}in", self.value),
            Unit::Length(Length::Pc) => write!(f, "{}pc", self.value),
            Unit::Length(Length::Pt) => write!(f, "{}pt", self.value),
            Unit::Length(Length::Px) => write!(f, "{}px", self.value),
            // Angle
            Unit::Angle(Angle::Deg) => write!(f, "{}deg", self.value),
            Unit::Angle(Angle::Grad) => write!(f, "{}grad", self.value),
            Unit::Angle(Angle::Rad) => write!(f, "{}rad", self.value),
            Unit::Angle(Angle::Turn) => write!(f, "{}turn", self.value),
            // Time
            Unit::Time(Time::S) => write!(f, "{}s", self.value),
            Unit::Time(Time::Ms) => write!(f, "{}ms", self.value),
            // Frequency
            Unit::Frequency(Frequency::Hz) => write!(f, "{}Hz", self.value),
            Unit::Frequency(Frequency::Khz) => write!(f, "{}kHz", self.value),
            // Resolution
            Unit::Resolution(Resolution::Dpi) => write!(f, "{}dpi", self.value),
            Unit::Resolution(Resolution::Dpcm) => write!(f, "{}dpcm", self.value),
            Unit::Resolution(Resolution::Dppx) => write!(f, "{}dppx", self.value),
        }
    }
}

fn display_length() {
    //
}

#[derive(Clone, Debug)]
pub enum Unit {
    Length(Length),
    Angle(Angle),
    Time(Time),
    Frequency(Frequency),
    Resolution(Resolution),
}

#[derive(Clone, Debug)]
pub enum Length {
    // Relative
    Em,
    Ex,
    Cap,
    Ch,
    Ic,
    Rem,
    Lh,
    Rlh,
    Vw,
    Vh,
    Vi,
    Vb,
    Vmin,
    Vmax,
    Percent,
    // Absolute
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt,
    Px,
}

// Relative
pub fn em<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Em),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn ex<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Ex),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn ch<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Ch),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn rem<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Rem),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn lh<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Lh),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn vw<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Vw),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn vh<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Vh),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn vmin<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Vmin),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn vmax<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Vmax),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn perc<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Percent),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Absolute
pub fn cm<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Cm),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn mm<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Mm),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn q<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Q),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn inch<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::In),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn pc<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Pc),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn pt<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Pt),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn px<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Length(Length::Px),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Angle
#[derive(Clone, Debug)]
pub enum Angle {
    Deg,
    Grad,
    Rad,
    Turn,
}

pub fn deg<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Angle(Angle::Deg),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn grad<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Angle(Angle::Grad),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn rad<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Angle(Angle::Rad),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn turn<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Angle(Angle::Turn),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Time
#[derive(Clone, Debug)]
pub enum Time {
    S,
    Ms,
}

pub fn s<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Time(Time::S),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn ms<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Time(Time::Ms),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Frequency
#[derive(Clone, Debug)]
pub enum Frequency {
    Hz,
    Khz,
}

pub fn hz<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Frequency(Frequency::Hz),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn khz<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Frequency(Frequency::Khz),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Resolution
#[derive(Clone, Debug)]
pub enum Resolution {
    Dpi,
    Dpcm,
    Dppx,
}

pub fn dpi<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Resolution(Resolution::Dpi),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn dpcm<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Resolution(Resolution::Dpcm),
        value: NotNan::new(val.into()).unwrap(),
    }
}

pub fn dppx<T: Into<f64>>(val: T) -> UnitValue {
    UnitValue {
        unit: Unit::Resolution(Resolution::Dppx),
        value: NotNan::new(val.into()).unwrap(),
    }
}

// Colors
pub fn hsl<H: Into<f64>, S: Into<f64>, L: Into<f64>>(h: H, s: S, l: L) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    CssColor::Hsl(h, s, l)
}

pub fn hsla<H: Into<f64>, S: Into<f64>, L: Into<f64>, A: Into<f64>>(
    h: H,
    s: S,
    l: L,
    a: A,
) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    let a = a.into();
    CssColor::Hsla(h, s, l, a)
}

pub fn hsluva<H: Into<f64>, S: Into<f64>, L: Into<f64>, A: Into<f64>>(
    h: H,
    s: S,
    l: L,
    a: A,
) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    let a = a.into();
    let rgb = hsluv::hsluv_to_rgb((h, s, l));
    CssColor::Rgba(rgb.0 * 255., rgb.1 * 255., rgb.2 * 255., a)
}

pub fn hsluv<H: Into<f64>, S: Into<f64>, L: Into<f64>>(h: H, s: S, l: L) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    let rgb = hsluv::hsluv_to_rgb((h, s, l));
    CssColor::Rgba(rgb.0 * 255., rgb.1 * 255., rgb.2 * 255., 1.0)
}

pub fn rgb<R: Into<f64>, G: Into<f64>, B: Into<f64>>(r: R, g: G, b: B) -> CssColor {
    let r = r.into();
    let g = g.into();
    let b = b.into();

    CssColor::Rgba(r, g, b, 1.0)
}

pub fn rgba<R: Into<f64>, G: Into<f64>, B: Into<f64>, A: Into<f64>>(
    r: R,
    g: G,
    b: B,
    a: A,
) -> CssColor {
    let r = r.into();
    let g = g.into();
    let b = b.into();
    let a = a.into();
    CssColor::Rgba(r, g, b, a)
}
