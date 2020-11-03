use std::f64::consts::PI;

use crate::style;
use crate::utils;

use pugl_sys::*;

pub struct LED {
    diameter: f64,
    hue: f64,

    on: bool
}

fn sanitize_hue(hue: f64) -> f64 {
    match hue {
        hue if hue < 0. => 0.,
        hue if hue > 1. => 0.,
        _ => hue
    }
}

impl LED {
    pub fn new(hue: f64) -> LED {
        LED {
            hue: sanitize_hue(hue),
            diameter: style::LED_DIAMETER,
            on: false
        }
    }

    pub fn set_hue(&mut self, hue: f64) -> &mut LED {
        if sanitize_hue(hue) == hue {
            self.hue = hue;
        }
        self
    }

    pub fn set_on(&mut self, on: bool) -> &mut LED {
        self.on = on;
        self
    }

    pub fn render(&self, cr: &cairo::Context, pos: Coord) {
        cr.save();
        cr.translate(pos.x, pos.y);

        cr.arc(0.0, 0.0, self.diameter/2., 0.0, 2.*PI);

        let v = if self.on {
            1.0
        } else {
            0.5
        };
        let (r, g, b) = utils::hsv_to_rgb(self.hue, 1., v);

        cr.set_source_rgb(r, g, b);
        cr.fill_preserve();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.stroke();

        cr.restore();
    }
}


#[cfg(all(test, feature="testing"))]
mod tests {
    use super::*;

    use crate::tests::SVGCairoTester;

    #[test]
    fn led_create() {
        let led = LED::new(0.0);
        assert!(!led.on);
        assert_eq!(led.hue, 0.0);
    }

    #[test]
    fn led_create_hue_valid_hue() {
        let led = LED::new(0.2);
        assert_eq!(led.hue, 0.2)
    }

    #[test]
    fn led_create_hue_gt1() {
        let led = LED::new(2.0);
        assert_eq!(led.hue, 0.0)
    }

    #[test]
    fn led_create_hue_lt0() {
        let led = LED::new(-2.0);
        assert_eq!(led.hue, 0.0)
    }

    #[test]
    fn led_set_hue_valid_hue() {
        let mut led = LED::new(0.3);
        led.set_hue(0.2);
        assert_eq!(led.hue, 0.2);
    }

    #[test]
    fn led_set_hue_gt1() {
        let mut led = LED::new(0.3);
        led.set_hue(2.0);
        assert_eq!(led.hue, 0.3)
    }

    #[test]
    fn led_set_hue_lt0() {
        let mut led = LED::new(0.3);
        led.set_hue(-2.0);
        assert_eq!(led.hue, 0.3)
    }

    #[test]
    fn led_draw_red_off() {
        let led = LED::new(0.0);
        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(50%,0%,0%);"));
    }

    #[test]
    fn led_draw_red_on() {
        let mut led = LED::new(0.0);
        led.set_on(true);

        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(100%,0%,0%);"));
    }

    #[test]
    fn led_draw_blue_off() {
        let mut led = LED::new(2./3.);
        led.set_on(false);

        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(0%,0%,50%);"));
    }

    #[test]
    fn led_draw_green_on() {
        let mut led = LED::new(1./3.);
        led.set_on(true);

        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(0%,100%,0%);"));
    }

    #[test]
    fn led_red_to_blue() {
        let mut led = LED::new(0.0);

        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(50%,0%,0%);"));

        led.set_hue(2./3.);

        let tester = SVGCairoTester::new(16., 16.);
        led.render(tester.context(), Coord { x: 8., y: 8. });

        assert!(tester.contents().contains("<path style=\"fill-rule:nonzero;fill:rgb(0%,0%,50%);"));
    }
}
