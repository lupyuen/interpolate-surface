// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::cg_vec_to_na;
use crate::constants::*;
use crate::delaunay_creation::{Delaunay, PointWithHeight};
use cgmath as cg;
use cgmath::EuclideanSpace;
use nalgebra as na;

// Interpolation Methods ------------------------------
pub trait InterpolationMethod {
    fn interpolate(d: &Delaunay, point: cg::Point2<f64>) -> f64;
    fn title() -> &'static str;
}

pub mod interpolation_methods {
    use super::InterpolationMethod;
    use crate::delaunay_creation::Delaunay;
    use cgmath as cg;

    pub struct BarycentricInterpolation;

    impl InterpolationMethod for BarycentricInterpolation {
        fn interpolate(delaunay: &Delaunay, point: cg::Point2<f64>) -> f64 {
            delaunay
                .barycentric_interpolation(&point, |v| v.height)
                .unwrap()
        }

        fn title() -> &'static str {
            "barycentric interpolation"
        }
    }

    pub struct NaturalNeighborInterpolation;

    impl InterpolationMethod for NaturalNeighborInterpolation {
        fn interpolate(delaunay: &Delaunay, point: cg::Point2<f64>) -> f64 {
            delaunay.nn_interpolation(&point, |v| v.height).unwrap()
        }

        fn title() -> &'static str {
            "natural neighbor interpolation"
        }
    }

    pub struct SibsonC1Interpolation;
    impl InterpolationMethod for SibsonC1Interpolation {
        fn interpolate(delaunay: &Delaunay, point: cg::Point2<f64>) -> f64 {
            delaunay
                .nn_interpolation_c1_sibson(
                    &point,
                    // Check out different smoothness factors
                    // 0.5,
                    // 2.0,
                    1.0,
                    // The second function defines the gradient of a point
                    |v| v.height,
                    |_, v| v.gradient,
                )
                .unwrap()
        }

        fn title() -> &'static str {
            "sibson's c1 interpolation"
        }
    }

    pub struct FarinC1Interpolation;
    impl InterpolationMethod for FarinC1Interpolation {
        fn interpolate(delaunay: &Delaunay, point: cg::Point2<f64>) -> f64 {
            delaunay
                .nn_interpolation_c1_farin(
                    &point,
                    // The second function defines the gradient of a point
                    |v| v.height,
                    |_, v| v.gradient,
                )
                .unwrap()
        }

        fn title() -> &'static str {
            "farin's c1 interpolation"
        }
    }
}

/*
 * Caches interpolated values on a grid and offers methods to
 * convert these into an edge list or a vertices / indices list
 */
pub struct Grid<I: InterpolationMethod> {
    grid: [[f64; X_PHYSICAL_SUBDIVISIONS + 1]; Y_PHYSICAL_SUBDIVISIONS + 1],
    __interpolation: ::std::marker::PhantomData<I>,
}

impl<I: InterpolationMethod> Grid<I> {
    // Returns a list of edges for rendering
    pub fn get_edges(&self) -> Vec<(na::Point3<f32>, na::Point3<f32>)> {
        let mut result = Vec::new();
        for y in 0..Y_PHYSICAL_SUBDIVISIONS {
            for x in 0..X_PHYSICAL_SUBDIVISIONS {
                let from_val = self.grid[y][x] + OFFSET;
                let from_pos = super::transform_physical_point(cg::Point2::new(x as f64, y as f64));
                let from = PointWithHeight::new(from_pos, from_val);
                for &(to_x, to_y) in &[(x + 1, y), (x, y + 1)] {
                    let to_val = self.grid[to_y][to_x] + OFFSET;
                    let to_pos = super::transform_physical_point(cg::Point2::new(to_x as f64, to_y as f64));
                    let to = PointWithHeight::new(to_pos, to_val);
                    result.push((
                        cg_vec_to_na(from.position_3d().to_vec().cast().unwrap()),
                        cg_vec_to_na(to.position_3d().to_vec().cast().unwrap()),
                    ));
                }
            }
        }
        result
    }

    // Returns a list of vertices and a list of triangle indices that form the
    // grid's mesh.
    pub fn get_triangles(&self) -> (Vec<na::Point3<f32>>, Vec<na::Point3<u16>>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for y in 0..=Y_PHYSICAL_SUBDIVISIONS {
            for x in 0..=X_PHYSICAL_SUBDIVISIONS {
                let val = self.grid[y][x] + OFFSET;
                let pos = super::transform_physical_point(cg::Point2::new(x as f64, y as f64));
                vertices.push(na::Point3::new(pos.x as f32, pos.y as f32, val as f32));
            }
        }
        for y in 0..Y_PHYSICAL_SUBDIVISIONS {
            for x in 0..X_PHYSICAL_SUBDIVISIONS {
                let index = |x, y| y * (X_PHYSICAL_SUBDIVISIONS + 1) + x;
                let v00 = index(x, y) as u16;
                let v10 = index(x + 1, y) as u16;
                let v01 = index(x, y + 1) as u16;
                let v11 = index(x + 1, y + 1) as u16;
                indices.push(na::Point3::new(v00, v10, v11));
                indices.push(na::Point3::new(v00, v11, v01));
            }
        }
        (vertices, indices)
    }

    // This will do the actual interpolation and store it in the triangulation
    #[allow(clippy::needless_range_loop)]
    pub fn from_delaunay_interpolation(delaunay: &Delaunay) -> Grid<I> {
        let mut values = [[0.0; X_PHYSICAL_SUBDIVISIONS + 1]; Y_PHYSICAL_SUBDIVISIONS + 1];
        for y in 0..=Y_PHYSICAL_SUBDIVISIONS {
            for x in 0..=X_PHYSICAL_SUBDIVISIONS {
                let pos = super::transform_physical_point(cg::Point2::new(x as f64, y as f64));
                let value = I::interpolate(delaunay, pos);
                values[y][x] = value.floor();

                //#[cfg(feature = "interpolate_x")]  //  If interpolating X values...
                //println!("XPhysical={:.0}, YPhysical={:.0}, XVirtual={:.0}", pos.x, pos.y, value);

                //#[cfg(feature = "interpolate_y")]  //  If interpolating X values...
                //println!("XPhysical={:.0}, YPhysical={:.0}, YVirtual={:.0}", pos.x, pos.y, value);
            }
        }

        //  Dump out the grid
        #[cfg(feature = "interpolate_x")]  //  If interpolating X values...
        println!("X_VIRTUAL_GRID=\n");
        
        #[cfg(feature = "interpolate_y")]  //  If interpolating Y values...
        println!("Y_VIRTUAL_GRID=\n");

        print!("[");
        for row in values.iter() {
            print!("[");
            for val in row.iter() {
                print!("{:.1},", val);
            }
            print!("],");
        }
        println!("]\n");

        Grid {
            grid: values,
            __interpolation: Default::default(),
        }
    }

}
