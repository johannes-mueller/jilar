
use pango;

use pugl_sys::*;
use pugl_ui::*;
use pugl_ui::ui::*;
use pugl_ui::widget::*;

use crate::led;
use crate::utils;

use crate::style;

const PADDING: f64 = 5.0;

pub struct Button {
    stub: WidgetStub,
    min_size: Size,
    text: String,

    clicked: bool,
    active: bool,

    toggle_state: Option<bool>,

    changed_toggle_state: Option<bool>,
}

impl Button {
    pub fn new_toggle_button(text: &str) -> Box<Button> {
        let mut btn = Self::new(text);
        btn.min_size.w += style::LED_DIAMETER * 2.0;
        btn.toggle_state = Some(false);
        btn
    }

    pub fn new (text: &str) -> Box<Button> {
        let sf = cairo::ImageSurface::create (cairo::Format::ARgb32, 8, 8).unwrap();
        let cr = cairo::Context::new (&sf);

        let lyt = utils::pango_layout(text, &cr);

        let (w, h) = lyt.get_pixel_size();
        let (w, h): (f64, f64) = (w.into(), h.into());

        let min_size: Size = Size { w: w + 2.*PADDING, h: h + 2.*PADDING };

        Box::new(Button {
            stub: WidgetStub::default(),
            text: String::from(text),
            min_size,
            clicked: false,
            active: false,
            toggle_state: None,
            changed_toggle_state: None
        })
    }

    pub fn clicked(&mut self) -> bool {
        let clicked = self.clicked;
        self.clicked = false;
        clicked
    }

    pub fn toggle_state(&self) -> Option<bool> {
        self.toggle_state
    }

    pub fn changed_toggle_state(&mut self) -> Option<bool> {
        self.changed_toggle_state.take()
    }

    pub fn set_toggle_state(&mut self, new_state: bool) {
        if self.toggle_state.is_some() {
            self.toggle_state = Some(new_state);
            self.ask_for_repaint();
        }
    }
}

impl Widget for Button {
    widget_stub!();

    fn exposed (&self, _expose: &ExposeArea, cr: &cairo::Context) {

        let size = self.size();
        let pos = self.pos();

        let (r, g, b) =  utils::widget_rgb(self.is_hovered(), None);
        cr.set_source_rgb(r, g, b);
        utils::rounded_rectangle(cr, pos, size, PADDING);
        //cr.fill_preserve();
        cr.set_line_width(0.75);
        cr.set_source_rgb(0., 0., 0.);
        cr.stroke();

        cr.save();
        cr.translate(pos.x, pos.y);

        if self.active {
            cr.set_source(&utils::active_gradient(size, (r, g, b)));
        } else {
            cr.set_source(&utils::inactive_gradient(size, (r, g, b)));
        }

        utils::rounded_rectangle(cr, Coord::default(), size, PADDING);
        cr.fill();
        cr.restore();

        match self.toggle_state {
            Some(t) => {
                let mut led = led::LED::new(1.0);
                if t {
                    led.set_on(true);
                }
                led.render(cr, Coord { x: pos.x + PADDING + style::LED_DIAMETER/2., y: pos.y + size.h/2. });
            }
            _ => {}
        }

        cr.save();
        cr.translate(pos.x + PADDING, pos.y + PADDING);
        if self.toggle_state.is_some() {
            cr.translate(style::LED_DIAMETER * 2., 0.0);
        }

        cr.set_source_rgb (1.0, 1.0, 1.0);

        let lyt = utils::pango_layout(&self.text, &cr);

        pangocairo::functions::show_layout (cr, &lyt);

        cr.restore();

        if self.has_focus() {
            cr.set_source_rgb (1., 1., 1.);
            cr.rectangle(pos.x, pos.y, size.w, size.h);
            cr.stroke();
        }
    }
    fn event (&mut self, ev: Event) -> Option<Event> {
        match ev.data {
            EventType::MouseMove(_mm) => {
                event_processed!()
            }
            EventType::MouseButtonPress(_btn) => {
                self.active = true;
                self.ask_for_repaint();
                event_processed!()
            },
            EventType::MouseButtonRelease(_btn) => {
                self.clicked = true;
                self.active = false;
                self.changed_toggle_state = self.toggle_state.and_then(|ts| {
                    Some(!ts)
                });

                event_processed!()
            },
            EventType::KeyRelease (ke) => {
                ke.try_char().and_then(|c| {
                    match c {
                        ' ' => {
                            event_processed!()
                        },
                        _ => event_not_processed!()
                    }
                }).or (event_not_processed!())
            },
            _ => event_not_processed!()
        }.and_then (|es| es.pass_event (ev))
    }
    fn min_size(&self) -> Size { self.min_size }

    fn takes_focus(&self) -> bool { true }
}
