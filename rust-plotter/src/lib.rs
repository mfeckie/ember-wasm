extern crate wee_alloc;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use plotters::prelude::*;
use std::ops::Range;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw Mandelbrot set on the provided canvas element.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn mandelbrot(canvas: HtmlCanvasElement) -> Result<Chart, JsValue> {
        let map_coord = draw(canvas).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }
}



/// Draw Mandelbrot set
pub fn draw(element: HtmlCanvasElement) -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>> {
    let backend = CanvasBackend::with_canvas_object(element).unwrap();

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_ranged(-2.1..0.6, -1.2..1.2)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();
    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (xr, yr) = (chart.x_range(), chart.y_range());

    for (x, y, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
        if c != 100 {
            plotting_area.draw_pixel((x, y), &HSLColor(c as f64 / 100.0, 1.0, 0.5))?;
        } else {
            plotting_area.draw_pixel((x, y), &BLACK)?;
        }
    }

    root.present()?;
    return Ok(Box::new(chart.into_coord_trans()));
}

fn mandelbrot_set(
    real: Range<f64>,
    complex: Range<f64>,
    samples: (usize, usize),
    max_iter: usize,
) -> impl Iterator<Item = (f64, f64, usize)> {
    let step = (
        (real.end - real.start) / samples.0 as f64,
        (complex.end - complex.start) / samples.1 as f64,
    );
    return (0..(samples.0 * samples.1)).map(move |k| {
        let c = (
            real.start + step.0 * (k % samples.0) as f64,
            complex.start + step.1 * (k / samples.0) as f64,
        );
        let mut z = (0.0, 0.0);
        let mut cnt = 0;
        while cnt < max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
            z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
            cnt += 1;
        }
        return (c.0, c.1, cnt);
    });
}
