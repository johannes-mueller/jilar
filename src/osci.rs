
use pugl_sys;
use pugl_ui::widget::*;

pub trait DrawingTask : Sync + Send {
    fn draw(&self, osci: &Osci, cr: &cairo::Context);
}

pub struct Osci {
    stub: WidgetStub,

    min_time: f64,
    max_time: f64,

    pub min_level: f64,
    pub max_level: f64,

    min_size: pugl_sys::Size,

    major_xticks: Vec<f64>,
    major_yticks: Vec<f64>,

    draw_tasks: Vec<Box<dyn DrawingTask>>
}

impl Osci {
    pub fn new() -> Box<Osci> {
        Box::new(Osci {
            stub: WidgetStub::default(),

            min_time: 0.0,
            max_time: 1.0,

            min_level: -1.0,
            max_level: 1.0,

            min_size: Default::default(),

            major_xticks: Vec::new(),
            major_yticks: Vec::new(),

            draw_tasks: Vec::new()
        })
    }

    pub fn set_time_range(&mut self, min: f64, max: f64) {
        self.min_time = min;
        self.max_time = max;
    }

    pub fn set_level_range(&mut self, min: f64, max: f64) {
        self.min_level = min;
        self.max_level = max;
    }

    pub fn set_min_width(&mut self, width: f64) {
        self.min_size.w = width;
    }

    pub fn set_min_height(&mut self, height: f64) {
        self.min_size.h = height;
    }

    pub fn linear_major_xticks(&mut self, number: u32) {
        self.major_xticks = make_linear_ticks(self.min_time, self.max_time, number);
    }

    pub fn linear_major_yticks(&mut self, number: u32) {
        self.major_yticks = make_linear_ticks(self.min_level, self.max_level, number);
    }

    pub fn scale_x(&self, x: f64) -> f64 {
        self.pos().x + (x - self.min_time) *  self.size().w / (self.max_time - self.min_time)
    }
    pub fn scale_y(&self, y: f64) -> f64 {
        self.pos().y + (self.max_level - y) *  self.size().h / (self.max_level - self.min_level)
    }

    pub fn submit_draw_task(&mut self, task: Box<dyn DrawingTask>) {
        self.draw_tasks.push(task);
    }
}

fn make_linear_ticks(min: f64, max: f64, number: u32) -> Vec<f64> {
    if min >= max && number == 0 {
        return Vec::new();
    }

    let step = (max - min) / number as f64;
    (1..number).map(|i| min + step * i as f64).collect()
}

impl Widget for Osci {
    widget_stub!();

    fn exposed(&mut self, _expose: &pugl_sys::ExposeArea, cr: &cairo::Context) {
        let size = self.size();
        let pos = self.pos();

        let (r, g, b) = (0.0, 0.0, 0.0);
        cr.set_source_rgb(r, g, b);
        cr.rectangle(pos.x, pos.y, size.w, size.h);
        cr.fill();

        cr.save();

        let x_min = self.scale_x(self.min_time);
        let x_max = self.scale_x(self.max_time);
        let y_min = self.scale_y(self.min_level);
        let y_max = self.scale_y(self.max_level);

        let (r, g, b) = (1.0, 1.0, 0.7);
        cr.set_source_rgb(r, g, b);
        cr.set_line_width(0.25);
        cr.set_dash(&[1., 2.], 0.0);

        for x in &self.major_xticks {
            let x = self.scale_x(*x);
            cr.move_to(x, y_min);
            cr.line_to(x, y_max);
            cr.stroke();
        }

        for y in &self.major_yticks {
            let y = self.scale_y(*y);
            cr.move_to(x_min, y);
            cr.line_to(x_max, y);
            cr.stroke();
        }

        cr.restore();

        cr.move_to(x_min, y_min);
        cr.line_to(x_min, y_max);
        cr.line_to(x_max, y_max);
        cr.line_to(x_max, y_min);
        cr.clip();
        for task in &self.draw_tasks {
            task.draw(self, cr);
        }
        cr.reset_clip();
    }

    fn min_size(&self) -> pugl_sys::Size {
        self.min_size
    }

    fn width_expandable(&self) -> bool { true }
    fn height_expandable(&self) -> bool { true }
}
