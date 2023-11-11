use leptos::html::Canvas;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use rand::Rng;
use std::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)) as f64
    }
}

#[derive(Clone)]
pub struct Angle {
    value: usize,
}

impl Angle {
    pub fn new(value: usize) -> Result<Self, String> {
        if value > 360 {
            Err("Angle must be less than 360".to_string())
        } else {
            Ok(Self { value })
        }
    }

    pub fn get(&self) -> usize {
        self.value
    }
}

pub fn rotate(cx: f32, cy: f32, x: f32, y: f32, angle: Angle) -> [f32; 2] {
    let radians: f32 = (PI as f32 / 180.0) * angle.get() as f32;
    let cos = radians.cos();
    let sin = radians.sin();
    let nx = (cos * (x - cx)) + (sin * (y - cy)) + cx;
    let ny = (cos * (y - cy)) - (sin * (x - cx)) + cy;
    [nx, ny]
}

pub fn get_canvas(canvas_id: String) -> web_sys::HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id.as_str()).unwrap();
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn get_canvas_context(
    canvas: &mut web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn scale_canvas(
    canvas: HtmlElement<Canvas>,
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
) {
    canvas
        .style("width", &format!("{}px", width))
        .style("height", &format!("{}px", height))
        .attr("width", width.floor())
        .attr("height", height.floor());

    context.scale(1.0, 1.0);
}

pub fn vec_pick<T: Clone>(my_vec: Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..my_vec.len());
    my_vec[index].clone()
}

pub fn chance(limit: Option<f32>) -> bool {
    let mut rng = rand::thread_rng();
    let chance: f64 = rng.gen();
    match limit {
        Some(limit) => chance as f32 <= limit,
        None => chance as f32 <= 0.5,
    }
}

pub fn get_rand_u8(min: u8, max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
