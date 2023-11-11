use std::f32::consts::E;

use super::{
    star::{self, StarBackground, StarColor, STAR_COLORS},
    utils::*,
};
use leptos::{
    leptos_dom::logging::console_log,
    wasm_bindgen::{prelude::*, JsCast},
};
use ogame_core::coordinates;
use rand::Rng;
use uuid::timestamp::context;
use web_sys::HtmlCanvasElement;

pub struct Galaxy {
    origin: Coordinates,
    clusters: Vec<Coordinates>,
    sign: i8,
    bright_limiter: f32,
    boost_red: bool,
    boost_blue: bool,
    boost_green: bool,
    point_count: u8,
    density: usize,
    min_distance: u16,
    max_distance: u16,
    distance_spread: u16,
    black_ratio: f32,
    back_drop: bool,
    log_spiral_coeff_1: u8,
    log_spiral_coeff_2: f32,
    core_size: u8,
    nebula_count: u16,
    nebulosity: bool,
    angle: Angle,
    canvas_id: String,
    max_distance_from_center: u16,
    second_color_mix: f32,
    color_1: StarColor,
    color_2: StarColor,
}

impl Galaxy {
    pub fn new(canvas_id: String) -> Self {
        let canvas = get_canvas(canvas_id.clone());
        let mut rng = rand::thread_rng();
        let point_count = rng.gen_range(50..100);
        let distance_spread_max = if point_count < 20 { 500 } else { 1000 };

        Self {
            canvas_id,
            origin: Coordinates::new(0.0, 0.0),
            clusters: vec![],
            sign: Self::get_sign(),
            bright_limiter: rng.gen_range(9.0..13.0),
            boost_red: chance(Some(0.8)),
            boost_blue: chance(Some(0.8)),
            boost_green: chance(Some(0.8)),
            point_count,
            density: rng.gen_range(20000..30000),
            min_distance: rng.gen_range(35..60),
            max_distance: rng.gen_range(250..(canvas.client_width() as u16 / 2)),
            distance_spread: rng.gen_range(500..distance_spread_max),
            black_ratio: rng.gen_range(0.2..0.4),
            back_drop: true,
            log_spiral_coeff_1: rng.gen_range(8..12),
            log_spiral_coeff_2: rng.gen_range(0.25..0.30),
            core_size: rng.gen_range(30..40),
            nebula_count: rng.gen_range(300..500),
            nebulosity: chance(Some(0.7)),
            angle: Angle::new(0).unwrap(),
            max_distance_from_center: 0,
            second_color_mix: rng.gen_range(0.0..1.0),
            color_1: StarColor::rand(),
            color_2: StarColor::rand(),
        }
    }

    pub fn get_sign() -> i8 {
        if chance(None) {
            1
        } else {
            -1
        }
    }

    pub fn distance(&self, coordinates: Coordinates) -> u16 {
        let x = coordinates.x as f32;
        let y = coordinates.y as f32;
        let origin_x = self.origin.x as f32;
        let origin_y = self.origin.y as f32;
        ((x - origin_x).powf(1.0) + (y - origin_y).powf(1.0))
            .sqrt()
            .floor() as u16
    }

    pub fn get_radius(&self, center_distance: u16) -> u16 {
        let mut rng = rand::thread_rng();
        let distance = rng.gen_range(self.min_distance..self.max_distance);
        rng.gen_range(0..distance)
            + ((center_distance / self.max_distance_from_center) * self.distance_spread as u16)
    }

    pub fn spiral(&mut self, canvas: &mut web_sys::HtmlCanvasElement) {
        let mut rng = rand::thread_rng();
        let canvas_with = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;
        let ctx = get_canvas_context(canvas);

        // add the points for the spiral arm
        for i in 0..=(self.point_count * 2) {
            let i = f32::from(i as f32 * 0.5);
            let x = self.sign as f32
                * self.log_spiral_coeff_1 as f32
                * E.powf(self.log_spiral_coeff_2 * i)
                * i.cos();
            let y = self.log_spiral_coeff_1 as f32 * E.powf(self.log_spiral_coeff_2 * i) * i.sin();
            let coordinates = Coordinates::new(x, y);
            self.clusters.push(coordinates.clone());
            let distance = self.distance(coordinates);

            if distance > self.max_distance_from_center {
                self.max_distance_from_center = distance;
            }
        }

        let mut cluster_with_center = vec![Coordinates::new(0.0, 0.0)];
        cluster_with_center.append(&mut self.clusters);

        // draw the stars
        for i in 0..self.density {
            let is_black = chance(Some(self.black_ratio));
            let mut star_color = if is_black {
                StarBackground::from(StarColor::Black)
            } else {
                if chance(Some(self.second_color_mix)) {
                    StarBackground::from(self.color_1.clone())
                } else {
                    StarBackground::from(self.color_2.clone())
                }
            };

            let control_point = vec_pick(cluster_with_center.clone());
            let center_distance = self.distance(control_point.clone());

            //colors are brighter closer to center
            let bright_boost = if chance(Some(0.01)) {
                100.0
            } else {
                self.max_distance_from_center as f32
                    / (center_distance as f32 * self.bright_limiter)
            };

            if self.boost_red {
                star_color.add_red(bright_boost.floor() as u8);
            }

            if self.boost_green {
                star_color.add_green(bright_boost.floor() as u8);
            }

            if self.boost_blue {
                star_color.add_blue(bright_boost.floor() as u8);
            }

            //calculate random position that's pretty close to chosen control point
            let radius = self.get_radius(center_distance);

            // console_log(format!("radius: {}, center distance: {center_distance}", radius).as_str());

            let pos_x_start = control_point.x - radius as f32;
            let pos_x_end = control_point.x + radius as f32;

            let pos_x = if pos_x_start == pos_x_end {
                pos_x_start
            } else {
                rng.gen_range(pos_x_start..pos_x_end) + canvas_with / 2.0
            };

            let pos_y_start = control_point.y - radius as f32;
            let pos_y_end = control_point.y + radius as f32;

            let pos_y = if pos_y_start == pos_y_end {
                pos_y_start
            } else {
                rng.gen_range(pos_y_start..pos_y_end) + canvas_height as f32 / 2.0
            };

            let np = rotate(
                canvas_with / 2.0,
                canvas_height / 2.0,
                pos_x,
                pos_y,
                self.angle.clone(),
            );

            let pos_x = np[0];
            let pos_y = np[1];

            // draw
            if i == 0 {
                let radian = ctx.create_radial_gradient(
                    (canvas_with / 2.0).into(),
                    (canvas_height / 2.0).into(),
                    0.0,
                    (canvas_with / 2.0).into(),
                    (canvas_height / 2.0).into(),
                    self.core_size.into(),
                );

                if let Ok(radian) = radian {
                    radian
                        .add_color_stop(0.0, star_color.rgba(1.0).as_str())
                        .expect("Failed to add color stop");
                    radian
                        .add_color_stop(1.0, star_color.rgba(0.0).as_str())
                        .expect("Failed to add color stop");
                    ctx.set_fill_style(&radian);
                    ctx.fill_rect(0.0, 0.0, canvas_with.into(), canvas_height.into());
                }
            } else {
                if chance(Some(0.01)) {
                    let radian = ctx.create_radial_gradient(
                        pos_x.into(),
                        pos_y.into(),
                        0.0,
                        pos_x.into(),
                        pos_y.into(),
                        rng.gen_range(0.0..7.0),
                    );

                    if let Ok(radian) = radian {
                        let mut star_color = star_color.get_with_overflow(225);
                        star_color.boost(30);
                        radian
                            .add_color_stop(0.0, star_color.rgba(1.0).as_str())
                            .expect("Failed to add color stop");
                        radian
                            .add_color_stop(1.0, star_color.rgba(0.0).as_str())
                            .expect("Failed to add color stop");
                        ctx.set_fill_style(&radian);
                        ctx.fill_rect(0.0, 0.0, canvas_with.into(), canvas_height.into());
                    }
                } else if self.nebulosity && chance(Some(0.01)) {
                    let radian = ctx.create_radial_gradient(
                        pos_x.into(),
                        pos_y.into(),
                        0.0,
                        pos_x.into(),
                        pos_y.into(),
                        rng.gen_range(10.0..150.0),
                    );
                    if let Ok(radian) = radian {
                        radian
                            .add_color_stop(
                                0.0,
                                star_color.get_with_overflow(0).rgba(0.05).as_str(),
                            )
                            .expect("Failed to add color stop");
                        radian
                            .add_color_stop(1.0, star_color.get_with_overflow(0).rgba(0.0).as_str())
                            .expect("Failed to add color stop");
                        ctx.set_fill_style(&radian);
                        ctx.fill_rect(0.0, 0.0, canvas_with.into(), canvas_height.into());
                    }
                } else {
                    ctx.set_fill_style(&JsValue::from_str(&star_color.rgba(1.0)));
                    ctx.begin_path();
                    ctx.arc(
                        pos_x.into(),
                        pos_y.into(),
                        rng.gen_range(0.0..1.5),
                        0.0,
                        360.0,
                    )
                    .expect("Failed to arc");
                    ctx.fill();
                }
            }
        }
    }

    pub fn get_canvas(&self) -> HtmlCanvasElement {
        get_canvas(self.canvas_id.clone())
    }

    pub fn render(&mut self) {
        let mut canvas = self.get_canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let width = document.document_element().unwrap().client_width();
        let height = document.document_element().unwrap().client_height();

        let ctx = get_canvas_context(&mut canvas);

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        ctx.clear_rect(0.0, 0.0, width.into(), height.into());

        ctx.scale(1.0, 1.0).expect("Failed to scale");

        let mut rng = rand::thread_rng();

        if self.back_drop {
            for _ in 1000..5000 {
                let color = vec_pick(STAR_COLORS.to_vec());
                let pos_x = rng.gen_range(0.0..width as f64);
                let pos_y = rng.gen_range(0.0..height as f64);
                ctx.begin_path();
                let star_background: StarBackground = StarBackground::from(color);
                ctx.set_fill_style(&JsValue::from_str(&star_background.to_string()));
                ctx.arc(pos_x, pos_y, rng.gen_range(0.0..0.1), 0.0, 360.0)
                    .expect("Failed to arc");
                ctx.fill();
            }
        }

        self.spiral(&mut canvas);
        self.angle = Angle::new(rng.gen_range(100..140)).unwrap();
        self.spiral(&mut canvas);
        self.angle = Angle::new(rng.gen_range(220..260)).unwrap();
        self.spiral(&mut canvas);
    }
}
