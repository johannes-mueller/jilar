

extern crate cairo;
extern crate pango;

#[macro_use]
extern crate cascade;

#[macro_use]
extern crate pugl_ui;

#[macro_use] mod utils;

pub mod label;
pub mod button;
pub mod dial;
pub mod osci;
pub mod meter;

pub use label::Label;
pub use button::Button;
pub use dial::Dial;
pub use osci::Osci;
pub use meter::Meter;

mod style;
mod led;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use crate::button;
    use crate::dial;
    use crate::label;
    use crate::osci;
    use crate::meter;

    use pugl_ui::ui;
    use pugl_ui::layout::StackDirection;
    use pugl_ui::widget;
    use pugl_ui::layout;
    use pugl_ui::widget::Widget;
    use pugl_sys::*;

    use cairo;

    #[derive(Default)]
    struct RootWidget {
        stub: widget::WidgetStub,

        rect1: widget::Layout,
        rect2: widget::Layout
    }

    impl widget::Widget for RootWidget {
        widget_stub!();

        fn exposed (&mut self, _expose: &pugl_sys::ExposeArea, cr: &cairo::Context) {
            cr.set_source_rgb (0.2, 0.2, 0.2);
            let size = self.size();
            cr.rectangle (0., 0., size.w, size.h);
            cr.fill ();

            cr.set_source_rgb(1., 1., 1.);
            let r = self.rect1;
            cr.rectangle(r.pos.x, r.pos.y, r.size.w, r.size.h);
            cr.stroke();

            let r = self.rect2;
            cr.rectangle(r.pos.x, r.pos.y, r.size.w, r.size.h);
            cr.stroke();
        }
    }

    impl RootWidget {
        fn draw_rects(&mut self, rect1: widget::Layout, rect2: widget::Layout) {
            self.rect1 = rect1;
            self.rect2 = rect2;
        }
    }

    struct OmegaDamp {
        omega_damp: Arc<RwLock<(f64, f64)>>
    }

    impl osci::DrawingTask for OmegaDamp {
        fn draw(&mut self, coord_system: osci::OsciCoordSystem, cr: &cairo::Context) {
            let (omega, damp) = *self.omega_damp.read().unwrap();
            cr.set_source_rgb(0.0, 1.0, 0.0);
            cr.set_line_width(1.0);
            cr.move_to(coord_system.scale_x(0.0), coord_system.scale_y(0.0));
            for i in 0..400 {
                let x = i as f64 / 400.0;
                let y = (x*omega).sin() * (-damp*x).exp();
                cr.line_to(coord_system.scale_x(x), coord_system.scale_y(y));
            }
            cr.stroke();
        }
    }

    #[test]
    fn showcase() {
        let mut ui = Box::new(ui::UI::new_scaled(Box::new(RootWidget::default()), 1.25));

        ui.layouter_handle(ui.root_layout()).set_padding(50.0);

        let dial_layout = ui.new_layouter::<layout::HorizontalLayouter>();
        let dial_v_lt1 = ui.new_layouter::<layout::VerticalLayouter>();
        let dial_v_lt2 = ui.new_layouter::<layout::VerticalLayouter>();
        let dial_v_lt3 = ui.new_layouter::<layout::VerticalLayouter>();

        let button = ui.new_widget(button::Button::new("Button"));
        let toggle_button = ui.new_widget(button::Button::new_toggle_button("ToggleButton", 0.66));

        let omega_damp = Arc::new(RwLock::new((90.0, 6.0)));

        let osci = ui.new_widget( cascade! {
            osci::Osci::new();
            ..set_min_height(180.0);
            ..set_min_width(320.0);
            ..linear_major_xticks(10);
            ..linear_major_yticks(10);
            ..submit_draw_task(Box::new(OmegaDamp { omega_damp: omega_damp.clone() }));
        });

        let meter = ui.new_widget(meter::Meter::new(1.0));

        let dial1 = ui.new_widget( cascade! {
            dial::Dial::<dial::LinearScale>::new(0., 180., 10);
            ..set_plate_draw( &|d: &dial::Dial<dial::LinearScale>, cr: &cairo::Context| { dial::draw_angle_tics(d, cr, 11) });
            ..set_hue(Some(0.1));
            ..set_default_value(90.0);
            ..set_value(90.0);
        });

        let dial2 = ui.new_widget( cascade! {
            dial::Dial::<dial::LogScale>::new(0.1, 1000., 10);
            ..set_plate_draw( &|d: &dial::Dial<dial::LogScale>, cr: &cairo::Context| { dial::draw_angle_tics(d, cr, 5) });
            ..set_hue(Some(0.7));
            ..set_value(6.0);
        });

        let dial3 = ui.new_widget( cascade! {
            dial::Dial::<dial::LinearScale>::new(-72.0, 24.0, 32);
            ..set_plate_draw( &|d: &dial::Dial<dial::LinearScale>, cr: &cairo::Context| { dial::draw_angle_tics(d, cr, 5) });
            ..set_hue(Some(0.4));
            ..set_value(-72.0);
        });

        let dial_big = ui.new_widget( cascade! {
            dial::Dial::<dial::LinearScale>::new(0., 1., 10);
            ..set_large();
        });

        let dial_small = ui.new_widget( cascade! {
            dial::Dial::<dial::LinearScale>::new(0., 1., 10);
            ..set_small();
        });

        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), ui.root_layout(), StackDirection::Back);
        ui.pack_to_layout(osci, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(meter, hl, StackDirection::Back);

        ui.pack_to_layout(button, ui.root_layout(), StackDirection::Back);
        ui.pack_to_layout(toggle_button, ui.root_layout(), StackDirection::Back);
        ui.pack_to_layout(dial_layout.widget(), ui.root_layout(), StackDirection::Back);

        ui.pack_to_layout(dial_v_lt1.widget(), dial_layout, StackDirection::Back);
        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt1, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(dial1, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt1, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        let lb = ui.new_widget(label::Label::new("maldekstra"));
        ui.pack_to_layout(lb, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        ui.pack_to_layout(dial_v_lt3.widget(), dial_layout, StackDirection::Back);
        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt3, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(dial3, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt3, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        let lb = ui.new_widget(label::Label::new("meza"));
        ui.pack_to_layout(lb, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        ui.pack_to_layout(dial_v_lt2.widget(), dial_layout, StackDirection::Back);
        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt2, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(dial2, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), dial_v_lt2, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        let lb = ui.new_widget(label::Label::new("dekstra"));
        ui.pack_to_layout(lb, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        let hl = ui.new_layouter::<layout::HorizontalLayouter>();
        ui.pack_to_layout(hl.widget(), ui.root_layout(), StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(dial_small, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);
        ui.pack_to_layout(dial_big, hl, StackDirection::Back);
        ui.add_spacer(hl, StackDirection::Back);

        ui.do_layout();

        let dial_medium_layout = ui.widget(dial_small).layout();
        let dial_big_layout = ui.widget(dial_big).layout();
        ui.root_widget().draw_rects(dial_medium_layout, dial_big_layout);

        let mut view = pugl_sys::PuglView::make_view(ui, std::ptr::null_mut());

        let ui = view.handle();

        ui.fit_window_size();
        ui.fit_window_min_size();
        ui.set_window_title("jilar widget showcase");
        //ui.make_resizable();
        ui.show_window();


        while !ui.close_request_issued() {
            ui.next_event(-1.0);

            let w = ui.widget(dial1);
            if let Some(v) = w.changed_value() {
                w.set_value(v);
                let mut omega_damp = omega_damp.write().unwrap();
                omega_damp.0 = v;
                ui.widget(osci).ask_for_repaint();
            }

            let w = ui.widget(dial2);
            if let Some(v) = w.changed_value() {
                w.set_value(v);
                let mut omega_damp = omega_damp.write().unwrap();
                omega_damp.1 = v;
                ui.widget(osci).ask_for_repaint();
            }

            let w = ui.widget(dial3);
            if let Some(v) = w.changed_value() {
                w.set_value(v);
                ui.widget(meter).set_level(v as f32);
            }

            let w = ui.widget(toggle_button);
            if let Some(ts) = w.changed_toggle_state() {
                w.set_toggle_state(ts);
            }
        }
    }
}
