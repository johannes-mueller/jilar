
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

pub use button::Button;
pub use dial::Dial;
pub use label::Label;

mod style;
mod led;

#[cfg(test)]
mod tests {
    use crate::button;
    use crate::dial;
    use crate::label;
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

        fn exposed (&self, _expose: &pugl_sys::ExposeArea, cr: &cairo::Context) {
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

    #[test]
    fn showcase() {
        let mut ui = Box::new(ui::UI::new(Box::new(RootWidget::default())));

        ui.layouter_handle(ui.root_layout()).set_padding(50.0);

        let dial_layout = ui.new_layouter::<layout::HorizontalLayouter>();
        let dial_v_lt1 = ui.new_layouter::<layout::VerticalLayouter>();
        let dial_v_lt2 = ui.new_layouter::<layout::VerticalLayouter>();

        let button = ui.new_widget(button::Button::new("Button"));
        let toggle_button = ui.new_widget(button::Button::new_toggle_button("ToggleButton"));

        let dial2 = ui.new_widget( cascade! {
            dial::Dial::new(0., 1., 0.1);
            ..set_plate_draw( &|d: &dial::Dial, cr: &cairo::Context| { dial::draw_angle_tics(d, cr, 11) });
            ..set_hue(Some(0.1));
        });

        let dial1 = ui.new_widget( cascade! {
            dial::Dial::new(0., 1., 0.1);
            ..set_plate_draw( &|d: &dial::Dial, cr: &cairo::Context| { dial::draw_angle_tics(d, cr, 5) });
            ..set_hue(Some(0.7));
        });

        let dial_big = ui.new_widget( cascade! {
            dial::Dial::new(0., 1., 0.1);
            ..set_large();
        });

        let dial_small = ui.new_widget( cascade! {
            dial::Dial::new(0., 1., 0.1);
            ..set_small();
        });

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

        let view = pugl_sys::PuglView::make_view(ui, std::ptr::null_mut());

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
            }

            let w = ui.widget(dial2);
            if let Some(v) = w.changed_value() {
                w.set_value(v);
            }

            let w = ui.widget(toggle_button);
            if let Some(ts) = w.changed_toggle_state() {
                w.set_toggle_state(ts);
            }
        }
    }
}
