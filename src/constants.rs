// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cgmath::Vector2;

pub const X_MIN: f64 = 0.0; // 3.5;
pub const X_MAX: f64 = 32.0; // 3.5;
pub const Y_MIN: f64 = 0.0; // 3.5;
pub const Y_MAX: f64 = 16.0; // 3.5;
//pub const SAMPLE_REGION: f64 = X_MAX - X_MIN; ////
//pub const SAMPLE_REGION: f64 = 32.0; // 3.5;
//pub const FREQUENCY: f64 = 1.;
//pub const NUM_POINTS: usize = 120;
//pub const MAX_HEIGHT: f64 = 1.5;

pub const GRID_SUBDIVISIONS: usize = 20; // 250;
pub const OFFSET: f64 = -0.01;
pub const GRID_WIDTH: f64 = (X_MAX - X_MIN) * 1.05;
pub const GRID_HEIGHT: f64 = (Y_MAX - Y_MIN) * 1.05;
//pub const GRID_SIZE: f64 = SAMPLE_REGION * 1.05;
pub const X_SCALE: f64 = 1.0 * GRID_WIDTH / (GRID_SUBDIVISIONS as f64);
pub const Y_SCALE: f64 = 1.0 * GRID_HEIGHT / (GRID_SUBDIVISIONS as f64);
//pub const SCALE: f64 = 1.0 * GRID_SIZE / (GRID_SUBDIVISIONS as f64);
//pub const SCALE: f64 = 2.0 * GRID_SIZE / (GRID_SUBDIVISIONS as f64);
pub const GRID_OFFSET: Vector2<f64> = Vector2 {
    x: X_MIN, // GRID_SIZE,
    y: Y_MIN, // GRID_SIZE,
};
