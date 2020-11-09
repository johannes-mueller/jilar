
use std::f64::consts::PI;

use pugl_sys::{Coord, Size};

use crate::style;

pub type RGB = (f64, f64, f64);

pub fn rounded_rectangle(cr: &cairo::Context, pos: Coord, size: Size, radius: f64) {
    const DEG: f64 = PI / 180.0;

    let r = radius;
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
    let ctx = pangocairo::functions::create_context(&cr).unwrap();
    let lyt = pango::Layout::new(&ctx);

    let font_desc = pango::FontDescription::from_string(style::BUTTONFONT);

    lyt.set_font_description(Some(&font_desc));
    lyt.set_text(&text);

    lyt
}


#[cfg(all(test, feature="testing"))]
mod tests {
    use super::*;

    use crate::tests::SVGCairoTester;

    use pango::*;


    #[test]
    fn rounded_rectangle_draw() {
        let tester = SVGCairoTester::new(16., 16.);

        rounded_rectangle(tester.context(), Coord::default(), Size { w: 16., h: 16.}, 4.);
        tester.context().stroke();

        let cont = tester.contents();
        assert!(cont.contains("d=\"M 16 12 C"));
        assert!(cont.contains("12 16 L 4 16 C"));
    }

    #[test]
    fn active_gradient_create() {
        let pos = Coord { x: 23., y: 42. };
        let size = Size { w: 12., h: 7. };

        let grad = active_gradient(pos, size, (0.96, 0.72, 0.48));

        let (x, y, w, h) = grad.get_linear_points();
        assert_eq!(x, 23.);
        assert_eq!(y, 42.);
        assert_eq!(w, 0.);
        assert_eq!(h, 7.);

        assert_eq!(grad.get_color_stop_count(), 2);

        assert_eq!(grad.get_color_stop_rgba(0), (0.0, 0.8, 0.6, 0.4, 1.0));
        assert_eq!(grad.get_color_stop_rgba(1), (0.5, 1.0, 1.2*0.72, 1.2*0.48, 1.0));
    }

    #[test]
    fn inactive_gradient_create() {
        let pos = Coord { x: 23., y: 42. };
        let size = Size { w: 12., h: 7. };

        let grad = inactive_gradient(pos, size, (0.96, 0.72, 0.48));

        let (x, y, w, h) = grad.get_linear_points();
        assert_eq!(x, 23.);
        assert_eq!(y, 42.);
        assert_eq!(w, 0.);
        assert_eq!(h, 7.);

        assert_eq!(grad.get_color_stop_count(), 2);

        assert_eq!(grad.get_color_stop_rgba(1), (0.5, 0.8, 0.6, 0.4, 1.0));
        assert_eq!(grad.get_color_stop_rgba(0), (0.0, 1.0, 1.2*0.72, 1.2*0.48, 1.0));
    }

    #[test]
    fn hue_to_rgb_plain_read() {
        let (h, s, v) = (1.0, 1.0, 1.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 1.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn hue_to_rgb_plain_blue() {
        let (h, s, v) = (2./3., 1.0, 1.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 0.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 1.0);
    }

    #[test]
    fn hue_to_rgb_plain_yellow() {
        let (h, s, v) = (1./6., 1.0, 1.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 1.0);
        assert_eq!(g, 1.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn hue_to_rgb_plain_black_red() {
        let (h, s, v) = (1.0, 1.0, 0.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 0.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn hue_to_rgb_plain_black_blue() {
        let (h, s, v) = (2./3., 0.5, 0.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 0.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn hue_to_rgb_plain_white_red() {
        let (h, s, v) = (1.0, 0.0, 1.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        assert_eq!(r, 1.0);
        assert_eq!(g, 1.0);
        assert_eq!(b, 1.0);
    }

    #[test]
    fn pango_layout_text() {
        let tester = SVGCairoTester::new(16., 16.);
        let lyt = pango_layout("test text", tester.context());
        assert_eq!(lyt.get_text().unwrap(), "test text");
    }

    #[test]
    fn pango_layout_font_description() {
        let tester = SVGCairoTester::new(16., 16.);
        let lyt = pango_layout("test text", tester.context());
        assert_eq!(lyt.get_font_description().unwrap().get_style(), Style::Normal);
        assert_eq!(lyt.get_font_description().unwrap().get_family().unwrap(), "Sans");
    }
}
