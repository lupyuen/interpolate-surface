// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Constants for interpolating CHIP-8's Emulator Screen to PineTime Display.
//! Since the X and Y axes are symmetric, we only compute one quadrant here (X >= 0, Y >= 0)
use cgmath::Vector2;

/// Range of Physical (x,y) coordinates, based on PineTime screen resolution
pub const X_PHYSICAL_MIN: f64 = 0.0;
pub const X_PHYSICAL_MAX: f64 = 120.0;
pub const Y_PHYSICAL_MIN: f64 = 0.0;
pub const Y_PHYSICAL_MAX: f64 = 100.0;

/// Range of Virtual (x,y) coordinates, based on CHIP-8 Emulator resolution
pub const X_VIRTUAL_MIN: f64 = 0.0;
pub const X_VIRTUAL_MAX: f64 = 32.0;
pub const Y_VIRTUAL_MIN: f64 = 0.0;
pub const Y_VIRTUAL_MAX: f64 = 16.0;

/// Virtual (x,y) coordinates shall be interpolated for Physical (x,y) coordinates at these increments
pub const X_PHYSICAL_INCREMENT: f64 = 1.0;  //  i.e. 0, 1, 2, ...
pub const Y_PHYSICAL_INCREMENT: f64 = 1.0;  //  i.e. 0, 1, 2, ...

/// Bounding Box for Physical (x,y) coordinates shall be computed for Virtual (x,y) coordinates at these increments
pub const X_VIRTUAL_INCREMENT: f64 = 1.0;  //  i.e. 0, 1, 2, ...
pub const Y_VIRTUAL_INCREMENT: f64 = 1.0;  //  i.e. 0, 1, 2, ...

/// How many divisions in the Physical X and Y axes to be interpolated
pub const X_PHYSICAL_SUBDIVISIONS: usize = ((X_PHYSICAL_MAX - X_PHYSICAL_MIN) / X_PHYSICAL_INCREMENT) as usize;
pub const Y_PHYSICAL_SUBDIVISIONS: usize = ((Y_PHYSICAL_MAX - Y_PHYSICAL_MIN) / Y_PHYSICAL_INCREMENT) as usize;

/// How many divisions in the Virtual X and Y axes to be computed for Bounding Box
pub const X_VIRTUAL_SUBDIVISIONS: usize = ((X_VIRTUAL_MAX - X_VIRTUAL_MIN) / X_VIRTUAL_INCREMENT) as usize;
pub const Y_VIRTUAL_SUBDIVISIONS: usize = ((Y_VIRTUAL_MAX - Y_VIRTUAL_MIN) / Y_VIRTUAL_INCREMENT) as usize;

/// Width and height of the PineTime Display
pub const PHYSICAL_WIDTH: f64 = (X_PHYSICAL_MAX - X_PHYSICAL_MIN) * 1.05;
pub const PHYSICAL_HEIGHT: f64 = (Y_PHYSICAL_MAX - Y_PHYSICAL_MIN) * 1.05;

/// Width and height of the CHIP-8 Emulator Display
pub const VIRTUAL_WIDTH: f64 = X_VIRTUAL_MAX - X_VIRTUAL_MIN;
pub const VIRTUAL_HEIGHT: f64 = Y_VIRTUAL_MAX - Y_VIRTUAL_MIN;

/// Scale the Physical points when interpolating
pub const X_PHYSICAL_SCALE: f64 = 1.0 * PHYSICAL_WIDTH / (X_PHYSICAL_SUBDIVISIONS as f64);
pub const Y_PHYSICAL_SCALE: f64 = 1.0 * PHYSICAL_HEIGHT / (Y_PHYSICAL_SUBDIVISIONS as f64);

/// Scale the Virtual points when computing bounding box
pub const X_VIRTUAL_SCALE: f64 = 1.0 * VIRTUAL_WIDTH / (X_VIRTUAL_SUBDIVISIONS as f64);
pub const Y_VIRTUAL_SCALE: f64 = 1.0 * VIRTUAL_HEIGHT / (Y_VIRTUAL_SUBDIVISIONS as f64);

/// Shift the Physical points when interpolating
pub const PHYSICAL_OFFSET: Vector2<f64> = Vector2 {
    x: X_PHYSICAL_MIN,  //  Previously GRID_SIZE
    y: Y_PHYSICAL_MIN,  //  Previously GRID_SIZE
};

/// Shift the Virtual points when computing bounding box
pub const VIRTUAL_OFFSET: Vector2<f64> = Vector2 {
    x: X_VIRTUAL_MIN,
    y: Y_VIRTUAL_MIN,
};

pub const OFFSET: f64 = 0.0;  //  Previously -0.01

//  Previously:
//  pub const SAMPLE_REGION: f64 = 3.5;
//  pub const FREQUENCY: f64 = 1.;
//  pub const NUM_POINTS: usize = 120;
//  pub const MAX_HEIGHT: f64 = 1.5;
//  pub const GRID_SUBDIVISIONS: usize = 20; // 250;
//  pub const GRID_SIZE: f64 = SAMPLE_REGION * 1.05;
//  pub const SCALE: f64 = 1.0 * GRID_SIZE / (GRID_SUBDIVISIONS as f64);
//  pub const SCALE: f64 = 2.0 * GRID_SIZE / (GRID_SUBDIVISIONS as f64);