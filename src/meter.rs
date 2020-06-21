use cairo;

use pugl_sys;
use pugl_ui::widget::*;

const MIN_LEVEL: f32 = -40.0;
const MAX_LEVEL: f32 = 12.0;

const METER_STEP: f32 = 1.0;

const MIN_WIDTH: f64 = 12.0;

pub struct Meter {
    stub: WidgetStub,
    current_level: f32,
    retained_level: Option<f32>,
}

impl Meter {
    pub fn new() -> Box<Meter> {
        Box::new(Meter {
            stub: WidgetStub::default(),
            current_level: -160.0,
            retained_level: None,
        })
    }

    pub fn set_level(&mut self, level: f32) {
        let new_level = level.max(MIN_LEVEL).min(MAX_LEVEL);
        if self.retained_level.map_or(true, |l| new_level > l) {
            self.retained_level = Some(new_level);
            self.request_reminder(3.0);
        }
        self.current_level = new_level;
        self.ask_for_repaint();
    }

    pub fn level(&self) -> f32 {
        self.current_level
    }
}

impl Widget for Meter {
    widget_stub!();

    fn exposed(&mut self, _expose: &pugl_sys::ExposeArea, cr: &cairo::Context) {
        let (left, right, top, bottom, width, height) = self.geometry();

        cr.set_source_rgb(1., 1., 1.);
        cr.set_line_width(1.0);
        cr.rectangle(left, top, width, height);
        cr.stroke();

        cr.set_source_rgb(0., 0., 0.);
        cr.rectangle(left, top, width, height);
        cr.fill();

        let db18 = scale_dB(height, -18.0);
        let db09 = scale_dB(height, -9.0);
        let db03 = scale_dB(height, -3.0);
        let db00 = scale_dB(height, -0.0);

        cr.set_source(&make_grad(left, top, right, (0.0, 0.15, 0.0)));
        cr.rectangle(left, bottom, width, -db18);
        cr.fill();

        cr.set_source(&make_grad(left, top, right, (0.0, 0.30, 0.0)));
        cr.rectangle(left, bottom - db18, width, db18 - db09);
        cr.fill();

        cr.set_source(&make_grad(left, top, right, (0.3, 0.3, 0.0)));
        cr.rectangle(left, bottom - db09, width, db09 - db03);
        cr.fill();

        cr.set_source(&make_grad(left, top, right, (0.3, 0.15, 0.0)));
        cr.rectangle(left, bottom - db03, width, db03 - db00);
        cr.fill();

        cr.set_source(&make_grad(left, top, right, (0.3, 0.0, 0.0)));
        cr.rectangle(left, bottom - db00, width, db00 - height);
        cr.fill();

        let level_height = self.current_level.min(-18.0);
        cr.set_source(&make_grad(left, top, right, (0.0, 0.5, 0.0)));
        cr.rectangle(left, bottom, width, -scale_dB(height, level_height));
        cr.fill();

        if self.current_level > -18.0 {
            let level_height = self.current_level.min(-9.0);
            cr.set_source(&make_grad(left, top, right, (0.0, 1.0, 0.0)));
            cr.rectangle(
                left,
                bottom - db18,
                width,
                db18 - scale_dB(height, level_height),
            );
            cr.fill();
        }

        if self.current_level > -9.0 {
            let level_height = self.current_level.min(-3.0);
            cr.set_source(&make_grad(left, top, right, (1.0, 1.0, 0.0)));
            cr.rectangle(
                left,
                bottom - db09,
                width,
                db09 - scale_dB(height, level_height),
            );
            cr.fill();
        }

        if self.current_level > -3.0 {
            let level_height = self.current_level.min(0.0);
            cr.set_source(&make_grad(left, top, right, (1.0, 0.5, 0.0)));
            cr.rectangle(
                left,
                bottom - db03,
                width,
                db03 - scale_dB(height, level_height),
            );
            cr.fill();
        }

        if self.current_level > 0.0 {
            let level_height = self.current_level.min(MAX_LEVEL);
            cr.set_source(&make_grad(left, top, right, (1.0, 0.0, 0.0)));
            cr.rectangle(
                left,
                bottom - db00,
                width,
                db00 - scale_dB(height, level_height),
            );
            cr.fill();
        }

        if let Some(level) = self.retained_level {
            let rgb = match level {
                l if l < -18.0 => (0., 0.5, 0.),
                l if l < -9.0 => (0., 1., 0.),
                l if l < -3.0 => (1., 1., 0.),
                l if l < 0.0 => (1., 0.5, 0.),
                _ => (1.0, 0.0, 0.0)
            };

            cr.set_source(&make_grad(left, top, right, rgb));
            let y = scale_dB(height, level);
            cr.rectangle(left, bottom - y, width, y - scale_dB(height, level - METER_STEP));
            cr.fill();
        }

        cr.set_source_rgb(0., 0., 0.);
        cr.set_line_width(1.0);
        let mut level = MIN_LEVEL + METER_STEP;
        while level < MAX_LEVEL {
            let y = bottom - scale_dB(height, level);
            cr.move_to(left, y);
            cr.line_to(right, y);
            level += METER_STEP;
        }
        cr.stroke();
    }

    fn min_size(&self) -> pugl_sys::Size {
        pugl_sys::Size { w: MIN_WIDTH, h: MIN_WIDTH * 5.0 }
    }

    fn height_expandable(&self) -> bool {
        true
    }

    fn reminder_handler(&mut self) {
        self.retained_level = None;
        self.ask_for_repaint();
    }
}

#[allow(non_snake_case)]
fn scale_dB(height: f64, level: f32) -> f64 {
    ((level.max(MIN_LEVEL) - MIN_LEVEL) / (MAX_LEVEL - MIN_LEVEL))
        .max(MIN_LEVEL)
        .min(MAX_LEVEL) as f64
        * height
}

fn make_grad(left: f64, top: f64, right: f64,
             rgb: (f64, f64, f64)) -> cairo::LinearGradient {
    let (r, g, b) = rgb;
    let grad = cairo::LinearGradient::new(left, top, right, top);

    grad.add_color_stop_rgb(0.0, r*0.5, g*0.5, b*0.5);
    grad.add_color_stop_rgb(0.5, r*1.0, g*1.0, b*1.0);
    grad.add_color_stop_rgb(1.0, r*0.5, g*0.5, b*0.5);

    grad
}
