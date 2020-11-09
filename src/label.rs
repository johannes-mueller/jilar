
use pugl_ui::widget::*;
use pugl_sys::*;

use crate::utils;

pub struct Label {
    stub: WidgetStub,
    text: String,
    min_size: Size
}

impl Label {
    pub fn new(text: &str) -> Box<Label> {
        let sf = cairo::ImageSurface::create (cairo::Format::ARgb32, 8, 8).unwrap();
        let cr = cairo::Context::new (&sf);

        let lyt = utils::pango_layout(text, &cr);

        let (w, h) = lyt.get_pixel_size();
        let (w, h): (f64, f64) = (w.into(), h.into());
        let min_size = Size { w, h };

        Box::new(Label {
            stub: WidgetStub::default(),
            text: text.to_string(),
            min_size
        })
    }
}

impl Widget for Label {
    widget_stub!();

    fn exposed(&mut self, _exposed: &ExposeArea, cr: &cairo::Context) {

        let pos = self.pos();

        cr.save();

        cr.set_source_rgb(1., 1., 1.);
        cr.translate(pos.x, pos.y);
        pangocairo::functions::show_layout(&cr, &utils::pango_layout(&self.text, cr));

        cr.restore();
    }

    fn min_size(&self) -> Size { self.min_size }
}

#[cfg(all(test, feature="testing"))]
mod tests {
    use super::*;

    #[test]
    fn label_create() {
        let label = Label::new("test label");

        assert_eq!(label.text, "test label");
    }
}
