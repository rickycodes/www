use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, html_element::CanvasElement, window, CanvasRenderingContext2d};
use util::{get_range, qs};

use stdweb::web::event::{MouseMoveEvent, ResizeEvent};

macro_rules! cursor {
    ($el:expr, $x:expr, $y:expr) => {
        $el.set_attribute(
            "style",
            &format!("transform: translate3d({},{},0);", $x, $y),
        )
        .unwrap();
    };
}

fn rect(context: &CanvasRenderingContext2d, x: f64, y: f64) {
    let width = get_range(4.0, 40.0);
    let height = get_range(4.0, 80.0);
    let alpha = get_range(0.001, 0.1);

    let stroke_style_color = format!("rgba(0, 0, 0, {})", alpha);

    context.begin_path();
    context.rect(
        f64::from(x as i32 - width as i32 / 2),
        f64::from(y as i32 - height as i32 / 2),
        width,
        height,
    );

    context.set_stroke_style_color(&stroke_style_color.to_string());
    context.stroke();
}

pub struct Canvas();

impl Canvas {
    pub fn new() -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(".canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        let resize_event = move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        };

        let mut throttle_count = 0;
        let throttle_every = 6;

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            throttle_count += 1;
            if throttle_count == throttle_every {
                rect(&context, x, y);
                throttle_count = 0;
            }

            qs(".coord > div").set_text_content(&format!("_x: {}, _y: {}", x, y));
            cursor!(qs(".x"), &format!("{}px", x), 0);
            cursor!(qs(".y"), 0, &format!("{}px", y));
            cursor!(qs(".cursor"), &format!("{}px", x), &format!("{}px", y));
        };

        window().add_event_listener(resize_event);
        window().add_event_listener(mouse_move_event);

        Canvas()
    }
}
