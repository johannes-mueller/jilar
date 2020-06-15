use std::f64::consts::PI;

use cairo;

use crate::style;
use crate::utils;

use pugl_sys::*;

pub struct LED {
    diameter: f64,
    hue: f64,

    on: bool
}

impl LED {
    pub fn new(hue: f64) -> LED {
        LED {
            hue,
            diameter: style::LED_DIAMETER,
            on: false
        }
    }

    pub fn set_hue(&mut self, hue: f64) -> &mut LED {
        self.hue = hue;
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

        let v = match self.on {
            true => 1.0,
            false => 0.5,
        };
        let (r, g, b) = utils::hsv_to_rgb(self.hue, 1., v);

        cr.set_source_rgb(r, g, b);
        cr.fill_preserve();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.stroke();

        cr.restore();
    }
}
