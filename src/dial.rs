use std::f64::consts::PI;

use pugl_ui::ui::*;
use pugl_ui::widget::*;
use pugl_sys::*;

use crate::utils;
use crate::style;

pub struct Dial {
    stub: WidgetStub,
    radius: f64,

    value: f64,
    default_value: Option<f64>,
    min_value: f64,
    max_value: f64,
    step: f64,

    changed_value: Option<f64>,

    drag_origin: Option<Coord>,
    drag_coeff: f64,

    hue: Option<f64>,
    value_indicator_active: bool,

    plate_drawer: &'static (dyn Fn(&Dial, &cairo::Context) + Sync),

    formater: &'static (dyn Fn(f64) -> String + Sync),
}

impl Dial {
    pub fn new(min_value: f64, max_value: f64, step: f64) -> Box<Dial> {
        Box::new(Dial {
            min_value, max_value, step,

            value: min_value,
            default_value: None,
            changed_value: None,

            drag_origin: None,
            drag_coeff: step * 0.1,

            value_indicator_active: false,
            hue: None,
            radius: style::DIAL_DIAMETER / 2.,

            plate_drawer: &|_dial, _cr| {},

            formater: &|value| format!("{:.1}", value),

            stub: WidgetStub::default()
        })
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
        self.ask_for_repaint();
    }

    pub fn set_default_value(&mut self, v: f64) {
        self.default_value = Some(v);
    }

    pub fn changed_value(&mut self) -> Option<f64> {
        self.changed_value.take()
    }

    pub fn set_hue(&mut self, hue: Option<f64>) {
        self.hue = hue;
    }

    pub fn set_plate_draw(&mut self, draw_func: &'static (dyn Fn(&Dial, &cairo::Context) + Sync)) {
        self.plate_drawer = draw_func;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_large(&mut self) {
        self.radius = style::DIAL_DIAMETER / 1.5;
    }

    pub fn set_small(&mut self) {
        self.radius = style::DIAL_DIAMETER / 3.0;
    }

    pub fn set_formater(&mut self, format_func: &'static (dyn Fn(f64) -> String + Sync)) {
        self.formater = format_func;
    }

    fn update_value(&mut self, diff: f64) {
        let nv = self.value + diff;
        let new_value = match nv {
            v if v > self.max_value => self.max_value,
            v if v < self.min_value => self.min_value,
            _ => nv
        };
        if new_value != self.value {
            self.changed_value = Some(new_value)
        }
    }
}


impl Widget for Dial {
    widget_stub!();

    fn exposed (&self, _exposed: &ExposeArea, cr: &cairo::Context) {

        let pos = self.pos() + Coord { x: 2.*self.radius, y: 2.*self.radius };
        cr.save();

        cr.translate(pos.x, pos.y + 6.0);

        cr.set_source(&utils::inactive_gradient(self.size(), utils::widget_rgb(self.is_hovered(), self.hue)));
        cr.arc(0., 0., self.radius * 0.8, 0.0, 2.*PI);
        cr.fill();

        cr.set_source_rgb(0., 0., 0.);
        cr.set_line_width(self.radius * 0.2);
        cr.arc(0., 0., self.radius, 0.0, 2.*PI);
        cr.stroke();

        let angle = 120. + 300. * (self.value-self.min_value)/(self.max_value-self.min_value);
        cr.set_source_rgb(1., 1., 1.);
        cr.set_line_width(self.radius * 0.2);
        cr.arc(0., 0., self.radius, (angle-10.0) * PI/180., (angle+10.0) * PI/180.);
        cr.stroke();

        cr.restore();
        cr.save();

        if self.value_indicator_active {
            let ctx = pangocairo::functions::create_context(&cr).expect("cration of pango context failed");
            let lyt = pango::Layout::new(&ctx);
            let font_desc = pango::FontDescription::from_string("Sans 8px");

            lyt.set_font_description(Some(&font_desc));
            lyt.set_text(&(self.formater)(self.value));

            let (ent, _) = lyt.get_extents();
            let (w, h) = ((ent.width/pango::SCALE) as f64, (ent.height/pango::SCALE) as f64);
            let bl = (lyt.get_baseline()/pango::SCALE) as f64;

            cr.translate(pos.x-w/2., pos.y-self.size().h/2. + h);
            cr.set_source_rgb(0., 0., 0.);
            cr.rectangle(0., 0., w, h+(bl/2.));
            cr.fill();
            cr.set_source_rgb(1., 1., 1.);
            pangocairo::functions::show_layout(cr, &lyt);
        }
        cr.restore();

        cr.save();
        cr.translate(pos.x - 2.*self.radius, pos.y -2.*self.radius);
        (self.plate_drawer)(self, cr);

        cr.restore();
    }

    fn event(&mut self, ev: Event) -> Option<Event> {
        match ev.data {
            EventType::Scroll (sc) => {
                let step = refine_step_by_modifiers(self.step, sc.modifiers);
                self.update_value(step * sc.dy.signum());
                event_processed!()
            }
            EventType::MouseButtonPress(btn) => {
                match btn.num {
                    1 => {
                        self.drag_origin = Some(ev.pos_root());
                        event_processed!()
                    }
                    3 => {
                        if let Some(default) = self.default_value {
                            if self.value != default {
                                self.changed_value = Some(default);
                            }
                        }
                        event_processed!()
                    }
                    _ => event_not_processed!()
                }
            }
            EventType::MouseButtonRelease(btn) => {
                match btn.num {
                    1 => {
                        self.drag_origin = None;
                        event_processed!()
                    }
                    _ => event_not_processed!()
                }
            }
            EventType::MouseMove(mm) => {
                match self.drag_origin {
                    Some(origin) => {
                        let pos = ev.pos_root();
                        let diff = (pos.x - origin.x) - (pos.y - origin.y);
                        let step = refine_step_by_modifiers(diff * self.drag_coeff, mm.modifiers);
                        self.update_value(step);
                        self.drag_origin = Some(pos);
                        event_processed!()
                    }
                    None => event_not_processed!()
                }
            }
            _ => event_not_processed!()
        }.and_then (|p| p.pass_event(ev))
    }

    fn pointer_enter(&mut self) {
        self.value_indicator_active = true;
        self.ask_for_repaint();
    }

    fn pointer_leave(&mut self) {
        self.value_indicator_active = false;
        self.ask_for_repaint();
    }

    fn min_size(&self) -> Size {
        Size { w: 4. * self.radius, h: 4. * self.radius + 12.0 }
    }
}

fn refine_step_by_modifiers(step: f64, modifiers: u32) -> f64 {
    if (Modifiers::from_bits_truncate(modifiers) & Modifiers::CTRL).is_empty() {
        step
    } else {
        step / 10.
    }
}

pub fn draw_angle_tics(dial: &Dial, cr: &cairo::Context, num: u32) {
    let rad = dial.radius() * 1.3;
    let size = dial.min_size();
    cr.set_source_rgb(1.,1.,1.);
    cr.translate(size.w/2., size.h/2.);
    cr.rotate(PI/2.);
    for _ in 0..num {
        cr.rotate(PI / (num as f64 + 1.0) * 2.);
        cr.move_to(rad, 0.0);
        cr.line_to(rad*1.2, 0.0);
        cr.stroke();
    }
}
