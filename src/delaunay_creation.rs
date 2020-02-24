// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//use rand::Rng;

use cgmath::{EuclideanSpace, Point2, Point3, Vector3};
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation};
use spade::HasPosition;

//use crate::constants::*;
//use noise::{NoiseFn, Seedable};

pub type Delaunay = FloatDelaunayTriangulation<PointWithHeight, DelaunayWalkLocate>;

pub struct PointWithHeight {
    point: Point2<f64>,
    pub height: f64,
    pub gradient: Point2<f64>,
    // We don't need the normal for interpolation purposes. We store it only for
    // visualization.
    pub normal: Vector3<f64>,
}

impl HasPosition for PointWithHeight {
    type Point = Point2<f64>;
    fn position(&self) -> Point2<f64> {
        self.point
    }
}

impl PointWithHeight {
    pub fn position_3d(&self) -> Point3<f64> {
        Point3::new(self.point.x, self.point.y, self.height)
    }

    pub fn new(point: Point2<f64>, height: f64) -> PointWithHeight {
        PointWithHeight {
            point,
            height,
            gradient: Point2::new(0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

// Triangulation creation and normal estimation
pub fn generate_random_triangulation() -> Delaunay {
    let mut delaunay = DelaunayTriangulation::with_walk_locate();

    /*
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 0 as f64), 0.0));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 0 as f64), 0.0));
    delaunay.insert(PointWithHeight::new(Point2::new(16 as f64, 8 as f64), 100.0));
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 16 as f64), 0.0));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 16 as f64), 0.0));
    */

    /*
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 0 as f64), 20.0));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 0 as f64), 40.0));
    delaunay.insert(PointWithHeight::new(Point2::new(16 as f64, 8 as f64), 100.0));
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 16 as f64), 60.0));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 16 as f64), 80.0));
    */

    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 0 as f64), 1.06194690265487 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(10.6666666666667 as f64, 0 as f64), 44.6017699115044 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(21.3333333333333 as f64, 0 as f64), 84.9557522123894 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 0 as f64), 120 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 5.33333333333333 as f64), 1.06194690265487 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(10.6666666666667 as f64, 5.33333333333333 as f64), 42.4778761061947 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(21.3333333333333 as f64, 5.33333333333333 as f64), 82.8318584070797 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 5.33333333333333 as f64), 118.938053097345 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 10.6666666666667 as f64), 1.06194690265487 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(10.6666666666667 as f64, 10.6666666666667 as f64), 38.2300884955752 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(21.3333333333333 as f64, 10.6666666666667 as f64), 76.4601769911505 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 10.6666666666667 as f64), 110.442477876106 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(0 as f64, 16 as f64), 1.06194690265487 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(10.6666666666667 as f64, 16 as f64), 31.858407079646 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(21.3333333333333 as f64, 16 as f64), 64.7787610619469 as f64));
    delaunay.insert(PointWithHeight::new(Point2::new(32 as f64, 16 as f64), 98.7610619469027 as f64));

    /*
    let mut rng = ::rand::thread_rng();
    let noise = ::noise::OpenSimplex::new().set_seed(rng.gen());
    for _ in 0..NUM_POINTS {
        let x = rng.gen_range(-SAMPLE_REGION, SAMPLE_REGION);
        let y = rng.gen_range(-SAMPLE_REGION, SAMPLE_REGION);
        let height = noise.get([x * FREQUENCY, y * FREQUENCY]) * MAX_HEIGHT;
        // Try out some other height functions, like those:
        // let height = (x * x + y * y) * 0.3;
        // let height = (x * 3.).sin() + (y - 2.).exp();
        delaunay.insert(PointWithHeight::new(Point2::new(x, y), height));
    }
    */

    // Note that, for interpolation, we only need the gradients. For visualization
    // purposes, the normals are also generated and stored within the vertices
    delaunay.estimate_gradients(&(|v| v.height), &(|v, g| v.gradient = g));
    delaunay.estimate_normals(
        &(|v| v.height),
        &(|v: &mut PointWithHeight, n: Point3<_>| v.normal = n.to_vec()),
    );

    delaunay
}
