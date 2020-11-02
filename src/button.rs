
use pugl_sys::*;
use pugl_ui::*;
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
    led_hue: f64,

    changed_toggle_state: Option<bool>,
}

impl Button {
    pub fn new_toggle_button(text: &str, led_hue: f64) -> Box<Button> {
        let mut btn = Self::new(text);
        btn.led_hue = led_hue;
        btn.min_size.w += style::LED_DIAMETER * 2.0;
        btn.toggle_state = Some(false);
        btn
    }

    pub fn new(text: &str) -> Box<Button> {
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
            led_hue: 0.0,
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

    fn exposed (&mut self, _expose: &ExposeArea, cr: &cairo::Context) {

        let size = self.size();
        let pos = self.pos();

        let (r, g, b) =  if self.is_hovered() {
            utils::hsv_to_rgb(0.0, 0.0, style::BRIGHTNESS_BUTTON_HOVER)
        } else {
            utils::hsv_to_rgb(0.0, 0.0, style::BRIGHTNESS_BUTTON_NORMAL)
        };
        cr.set_source_rgb(r, g, b);
        utils::rounded_rectangle(cr, pos, size, PADDING);
        //cr.fill_preserve();
        cr.set_line_width(2.0);
        cr.set_source_rgb(0., 0., 0.);
        cr.stroke();

        cr.save();
        cr.translate(pos.x, pos.y);

        let null_pos = Coord { x:0.0, y: 0.0 };

        if self.active {
            cr.set_source(&utils::active_gradient(null_pos, size, (r, g, b)));
        } else {
            cr.set_source(&utils::inactive_gradient(null_pos, size, (r, g, b)));
        }

        utils::rounded_rectangle(cr, Coord::default(), size, PADDING);
        cr.fill();
        cr.restore();

        if let Some(ts) = self.toggle_state {
            let mut led = led::LED::new(self.led_hue);
            if ts {
                led.set_on(true);
            }
            led.render(cr, Coord { x: pos.x + PADDING + style::LED_DIAMETER/2., y: pos.y + size.h/2. });
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
                self.changed_toggle_state = self.toggle_state.map(|ts| !ts);

                event_processed!()
            },
            EventType::KeyPress(ke) => {
                ke.try_char().and_then(|c| {
                    match c {
                        ' ' => {
                            self.active = true;
                            event_processed!()
                        },
                        _ => event_not_processed!()
                    }
                }).or (event_not_processed!())
            },
            EventType::KeyRelease(ke) => {
                ke.try_char().and_then(|c| {
                    match c {
                        ' ' => {
                            self.clicked = true;
                            self.active = false;
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

#[cfg(all(test, feature="testing"))]
mod tests {
    use super::*;
    use pugl_ui::ui::*;
    use pugl_ui::layout::stacklayout::*;

    #[derive(Default)]
    struct RootWidget {
        stub: WidgetStub
    }

    impl Widget for RootWidget {
        widget_stub!();
    }

    #[test]
    fn test_button_click() {
        let rw = Box::new(RootWidget::default());
        let mut view = PuglView::new(std::ptr::null_mut(), |pv| UI::new_scaled(pv, rw, 1.0));

        let (click_pos, button) = {
            let ui = view.handle();
            let button = ui.new_widget(Button::new("test button"));
            ui.pack_to_layout(button, ui.root_layout(), StackDirection::Front);
            ui.do_layout();
            ui.show_window();

            let w = ui.widget(button);
            let pos = w.pos();
            let size = w.size();
            (Coord { x: pos.x + size.w/2., y: pos.y + size.h/2. }, button)
        };

        view.queue_event(Event {
            data: EventType::MouseButtonRelease(MouseButton { num: 1, modifiers: Modifiers::default() }),
            context: EventContext { pos: click_pos, ..Default::default() }
        });

        let ui = view.handle();

        assert!(!ui.widget(button).clicked());
        ui.update(-1.0);
        assert!(ui.widget(button).clicked());
    }

    #[test]
    fn test_button_space_key_click() {
        let rw = Box::new(RootWidget::default());
        let mut view = PuglView::new(std::ptr::null_mut(), |pv| UI::new_scaled(pv, rw, 1.0));

        view.queue_event(Event {
            data: EventType::KeyRelease(Key { key: KeyVal::Character(' '), modifiers: Modifiers::default(), code: 65 }),
            context: EventContext::default()
        });

        let ui = view.handle();
        let button = ui.new_widget(Button::new("test button"));
        ui.pack_to_layout(button, ui.root_layout(), StackDirection::Front);
        ui.do_layout();
        ui.show_window();

        ui.focus_widget(button);

        assert!(!ui.widget(button).clicked());
        ui.update(-1.0);
        assert!(ui.widget(button).clicked());
    }

    #[test]
    fn test_button_active() {
        let rw = Box::new(RootWidget::default());
        let mut view = PuglView::new(std::ptr::null_mut(), |pv| UI::new_scaled(pv, rw, 1.0));

        let (click_pos, button) = {
            let ui = view.handle();
            let button = ui.new_widget(Button::new("test button"));
            ui.pack_to_layout(button, ui.root_layout(), StackDirection::Front);
            ui.do_layout();
            ui.show_window();

            let w = ui.widget(button);
            let pos = w.pos();
            let size = w.size();
            (Coord { x: pos.x + size.w/2., y: pos.y + size.h/2. }, button)
        };

        view.queue_event(Event {
            data: EventType::MouseButtonPress(MouseButton { num: 1, modifiers: Modifiers::default() }),
            context: EventContext { pos: click_pos, ..Default::default() }
        });
        view.queue_event(Event {
            data: EventType::MouseButtonRelease(MouseButton { num: 1, modifiers: Modifiers::default() }),
            context: EventContext { pos: click_pos, ..Default::default() }
        });

        let ui = view.handle();

        assert!(!ui.widget(button).active);
        ui.update(-1.0);
        assert!(ui.widget(button).active);
        ui.update(-1.0);
        assert!(!ui.widget(button).active);
    }

    #[test]
    fn test_button_space_key_active() {
        let rw = Box::new(RootWidget::default());
        let mut view = PuglView::new(std::ptr::null_mut(), |pv| UI::new_scaled(pv, rw, 1.0));

        view.queue_event(Event {
            data: EventType::KeyPress(Key { key: KeyVal::Character(' '), modifiers: Modifiers::default(), code: 65 }),
            context: EventContext::default()
        });
        view.queue_event(Event {
            data: EventType::KeyRelease(Key { key: KeyVal::Character(' '), modifiers: Modifiers::default(), code: 65 }),
            context: EventContext::default()
        });

        let ui = view.handle();
        let button = ui.new_widget(Button::new("test button"));
        ui.pack_to_layout(button, ui.root_layout(), StackDirection::Front);
        ui.do_layout();
        ui.show_window();

        ui.focus_widget(button);

        assert!(!ui.widget(button).active);
        ui.update(-1.0);
        assert!(ui.widget(button).active);
        ui.update(-1.0);
        assert!(!ui.widget(button).active);
    }
}
