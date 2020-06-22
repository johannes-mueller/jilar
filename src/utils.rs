
use std::f64::consts::PI;

use cairo;
use pugl_sys::{Coord, Size};

use crate::style;

pub type RGB = (f64, f64, f64);

pub fn rounded_rectangle(cr: &cairo::Context, pos: Coord, size: Size, r: f64) {
    const DEG: f64 = PI / 180.0;

    let (x, y) = (pos.x, pos.y);
    let (w, h) = (size.w, size.h);

    cr.new_sub_path();
    cr.arc(x + w - r, y + h - r, r,   0.0 * DEG,  90.0 * DEG);
    cr.arc(x + r    , y + h - r, r,  90.0 * DEG, 180.0 * DEG);
    cr.arc(x + r    , y + r, r, 180.0 * DEG, 270.0 * DEG);
    cr.arc(x + w - r, y + r , r, 270.0 * DEG,   0.0 * DEG);
    cr.close_path();
}

pub fn active_gradient(pos: Coord, size: Size, rgb: (f64, f64, f64)) -> cairo::LinearGradient {
    let (r, g, b) = rgb;
    let grad = cairo::LinearGradient::new(pos.x, pos.y, 0.0, size.h);

    grad.add_color_stop_rgb(0.5, r*1.2, g*1.2, b*1.2);
    grad.add_color_stop_rgb(0.0, r/1.2, g/1.2, b/1.2);

    grad
}


pub fn inactive_gradient(pos: Coord, size: Size, rgb: (f64, f64, f64)) -> cairo::LinearGradient {
    let (r, g, b) = rgb;
    let grad = cairo::LinearGradient::new(pos.x, pos.y, 0.0, size.h);

    grad.add_color_stop_rgb(0.0, r*1.2, g*1.2, b*1.2);
    grad.add_color_stop_rgb(0.5, r/1.2, g/1.2, b/1.2);

    grad
}

pub fn widget_rgb(hovered: bool, hue: Option<f64>) -> RGB {
    let v = if hovered {
        style::BRIGHTNESS_HOVER
    } else {
        style::BRIGHTNESS_NORMAL
    };

    let (h, s) = hue.map_or((1.0, 0.0), |h| (h, style::WIDGET_COLOR_SAT));
    hsv_to_rgb(h, s, v)
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> RGB {
    let c = v * s;

    const N: f64 = 0.0;

    let h6 = h*6.;
    let x = c * (1. - (h6 % 2. - 1.).abs());
    let (r, g, b) = match h6 {
        h if h >= 0.0 && h < 1.0 => (c, x, N),
        h if h >= 1.0 && h < 2.0 => (x, c, N),
        h if h >= 2.0 && h < 3.0 => (N, c, x),
        h if h >= 3.0 && h < 4.0 => (N, x, c),
        h if h >= 4.0 && h < 5.0 => (x, N, c),
        h if h >= 5.0 && h <= 6.0 => (c, N, x),
        _ => (N, N, N)
    };

    let m = v - c;
    (r+m, g+m, b+m)
}


pub fn pango_layout(text: &str, cr: &cairo::Context) -> pango::Layout {
    let ctx = pangocairo::functions::create_context (&cr).unwrap();
    let lyt = pango::Layout::new (&ctx);

    let font_desc = pango::FontDescription::from_string (style::BUTTONFONT);

    lyt.set_font_description (Some(&font_desc));
    lyt.set_text (&text);

    lyt
}
