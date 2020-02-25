// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Interpolate CHIP-8's Emulator Screen (square) to PineTime Display (spherical).
//! Since the X and Y axes are symmetric, we only compute one quadrant here (X >= 0, Y >= 0)
/*
 * Press h for help.
 * See ./interpolation.rs for interpolation related code and
 * ./delaunay_creation.rs for loading the data points.
 *
 * *Note*: This demo uses kiss3d which uses an old version of
 * nalgebra. This nalgebra version is incompatible with spade, that's
 * the reason why we're using cgmath points in the delaunay triangulation
 * and nalgebra points for the rendering in kiss3d. Once kiss3d updates,
 * only nalgebra will be used.
 */
#![feature(assoc_int_consts)]  //  Allow f64::MIN and MAX
#![warn(clippy::all)]
extern crate cgmath;
extern crate kiss3d;
extern crate nalgebra;
extern crate noise;
extern crate rand;
extern crate spade;

mod constants;
mod delaunay_creation;
mod interpolation;
mod data;

use nalgebra as na;

use cgmath as cg;
use cgmath::EuclideanSpace;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use std::cell::RefCell;
use std::rc::Rc;

use crate::constants::*;
use crate::delaunay_creation::Delaunay;
use crate::interpolation::interpolation_methods::{
    BarycentricInterpolation, FarinC1Interpolation, NaturalNeighborInterpolation,
    SibsonC1Interpolation,
};
use crate::interpolation::{Grid, InterpolationMethod};

struct InterpolationRenderData {
    edges: Vec<(na::Point3<f32>, na::Point3<f32>)>,
    mesh: Rc<RefCell<Mesh>>,
    title: &'static str,
}

pub fn cg_vec_to_na(vec: cg::Vector3<f64>) -> na::Point3<f32> {
    na::Point3::new(vec.x as f32, vec.y as f32, vec.z as f32)
}

impl InterpolationRenderData {
    fn new<I: InterpolationMethod>(delaunay: &Delaunay) -> InterpolationRenderData {
        println!("{}", I::title());
        let grid = Grid::<I>::from_delaunay_interpolation(delaunay);
        let (vertices, indices) = grid.get_triangles();
        let mesh = Mesh::new(vertices, indices, None, None, false);
        InterpolationRenderData {
            edges: grid.get_edges(),
            mesh: Rc::new(RefCell::new(mesh)),
            title: I::title(),
        }
    }
}

fn print_help() {
    println!("Interpolation Demo");
    println!("H - print this help");
    println!("N - show / hide normals");
    println!("G - switch interpolation method");
    println!("D - show / hide delaunay triangulation");
    println!("T - toggle display method of interpolated mesh");
}

#[derive(PartialEq, Eq)]
enum DelaunayVisibility {
    All,
    OnlyLines,
    None,
}

#[derive(PartialEq, Eq)]
enum GridRenderType {
    Lines,
    Polygons,
}

impl DelaunayVisibility {
    fn next(&self) -> DelaunayVisibility {
        use crate::DelaunayVisibility::*;
        match self {
            All => OnlyLines,
            OnlyLines => None,
            None => All,
        }
    }
}

impl GridRenderType {
    fn next(&self) -> GridRenderType {
        use crate::GridRenderType::*;
        match self {
            Lines => Polygons,
            Polygons => Lines,
        }
    }
}

fn main() {
    let mut window = Window::new("Delaunay Demo");
    window.set_light(Light::StickToCamera);

    print_help();

    let mut delaunay_visibility = DelaunayVisibility::All;
    let mut grid_render_type = GridRenderType::Lines;
    let mut show_normals = false;

    // Create delaunay triangulation and its mesh
    let delaunay = crate::delaunay_creation::generate_random_triangulation();
    let delaunay_mesh = create_mesh_from_triangulation(&delaunay);
    let delaunay_mesh = Rc::new(RefCell::new(delaunay_mesh));
    let mut delaunay_node = window.add_mesh(delaunay_mesh.clone(), na::Vector3::new(1.0, 1.0, 1.0));
    delaunay_node.enable_backface_culling(false);
    let delaunay_lines = extract_edges(&delaunay);

    let interpolation_meshes = [
        InterpolationRenderData::new::<BarycentricInterpolation>(&delaunay),
        InterpolationRenderData::new::<NaturalNeighborInterpolation>(&delaunay),
        InterpolationRenderData::new::<SibsonC1Interpolation>(&delaunay),
        InterpolationRenderData::new::<FarinC1Interpolation>(&delaunay),
    ];

    let mut cur_interpolation_mesh_node: Option<SceneNode> = None;
    let mut cur_interpolation_mesh_index = 0;

    let normals = get_normals(&delaunay);

    generate_virtual_to_physical_map();

    while window.render() {
        for event in window.events().iter() {
            let mut update_interpolation_mesh = false;
            match event.value {
                WindowEvent::Key(Key::H, Action::Press, _) => print_help(),
                WindowEvent::Key(Key::N, Action::Press, _) => show_normals = !show_normals,
                WindowEvent::Key(Key::G, Action::Press, _) => {
                    cur_interpolation_mesh_index += 1;
                    update_interpolation_mesh = true;
                    if cur_interpolation_mesh_index > interpolation_meshes.len() {
                        cur_interpolation_mesh_index = 0;
                    }
                    if cur_interpolation_mesh_index < interpolation_meshes.len() {
                        println!(
                            "Change interpolation method to {}",
                            interpolation_meshes[cur_interpolation_mesh_index].title
                        );
                    }
                }
                WindowEvent::Key(Key::T, Action::Press, _) => {
                    grid_render_type = grid_render_type.next();
                    update_interpolation_mesh = true;
                }
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    delaunay_visibility = delaunay_visibility.next();
                    if delaunay_visibility == DelaunayVisibility::All {
                        delaunay_node = window
                            .scene_mut()
                            .add_mesh(delaunay_mesh.clone(), na::Vector3::new(1.0, 1.0, 1.));
                        delaunay_node.enable_backface_culling(false);
                    } else {
                        delaunay_node.unlink();
                    }
                }
                _ => {}
            }
            if update_interpolation_mesh {
                if let Some(mut mesh_node) = cur_interpolation_mesh_node {
                    mesh_node.unlink();
                    cur_interpolation_mesh_node = None;
                }
                if cur_interpolation_mesh_index < interpolation_meshes.len()
                    && grid_render_type == GridRenderType::Polygons
                {
                    let mut new_node = window.scene_mut().add_mesh(
                        interpolation_meshes[cur_interpolation_mesh_index]
                            .mesh
                            .clone(),
                        na::Vector3::new(1.0, 1.0, 1.0),
                    );
                    new_node.enable_backface_culling(false);
                    cur_interpolation_mesh_node = Some(new_node);
                }
            }
        }

        if delaunay_visibility == DelaunayVisibility::All
            || delaunay_visibility == DelaunayVisibility::OnlyLines
        {
            let color = na::Point3::new(0.8, 0.5, 0.2);
            for &(from, to) in &delaunay_lines {
                window.draw_line(&from, &to, &color);
            }
        }

        if grid_render_type == GridRenderType::Lines {
            if let Some(mesh) = interpolation_meshes.get(cur_interpolation_mesh_index) {
                let color = na::Point3::new(0.5, 0.8, 0.2);
                for &(from, to) in &mesh.edges {
                    window.draw_line(&from, &to, &color);
                }
            }
        }

        if show_normals {
            let color = na::Point3::new(0.5, 0.5, 1.0);
            for &(from, to) in &normals {
                window.draw_line(&from, &to, &color);
            }
        }
    }
}

fn generate_virtual_to_physical_map() {
    //  For all Virtual (x,y) Coordinates, compute the Bounding Box: min and max of Physical x or y Coordinates
    for y in 0..=Y_VIRTUAL_SUBDIVISIONS {
        for x in 0..=X_VIRTUAL_SUBDIVISIONS {
            //  Convert the normalised (x,y) into Virtual (x,y) Coordinates
            let pos = transform_virtual_point(cg::Point2::new(x as f64, y as f64));
            //  For all Physical (x,y) that interpolate to the Virtual (x,y), find the bounding box
            let bounding_box = get_bounding_box(
                data::X_VIRTUAL_GRID,
                data::Y_VIRTUAL_GRID,
                pos.x,
                pos.y);  //  Returns (left, top, right, bottom) for the Bounding Box
            if let Some((left, top, right, bottom)) = bounding_box {
                if left as u8 == right as u8 && top as u8 == bottom as u8 {
                    print!("****");  //  Flag out Virtual Points that map to a single Physical Point
                }
            }
            println!("XVirtual={:.0}, YVirtual={:.0}, BoundBox={:.?}", pos.x, pos.y, bounding_box);
        }
    }
}

/// Given a grid of Physical (x,y) Coordinates and their interpolated Virtual (x,y) Coordinates, 
/// find all Physical (x,y) Coordinates that interpolate to (x_virtual,y_virtual).
/// Return the (left, top, right, bottom) of the Bounding Box that encloses these found points.
/// x_virtual and y_virtual are truncated to integer during comparison.
/// Function returns `None` if (x_virtual,y_virtual) was not found.
fn get_bounding_box(
    x_virtual_grid: &[[f64; X_PHYSICAL_SUBDIVISIONS + 1]; Y_PHYSICAL_SUBDIVISIONS + 1],
    y_virtual_grid: &[[f64; X_PHYSICAL_SUBDIVISIONS + 1]; Y_PHYSICAL_SUBDIVISIONS + 1],
    x_virtual: f64,
    y_virtual: f64
) -> Option<(f64, f64, f64, f64)> {
    let mut left: f64 = f64::MAX;
    let mut top: f64 = f64::MAX;
    let mut right: f64 = f64::MIN;
    let mut bottom: f64 = f64::MIN;
    //  For all Physical (x,y) Coordinates...
    for y in 0..=Y_PHYSICAL_SUBDIVISIONS {
        for x in 0..=X_PHYSICAL_SUBDIVISIONS {
            //  Get the Physical (x,y) Coordinates
            let pos = transform_physical_point(cg::Point2::new(x as f64, y as f64));

            //  Get the interpolated Virtual (x,y) Coordinates
            let x_interpolated = x_virtual_grid[y][x].floor();
            let y_interpolated = y_virtual_grid[y][x].floor();

            //  Skip if not matching
            if x_interpolated as u8 != x_virtual as u8 || 
                y_interpolated as u8 != y_virtual as u8 { continue; }

            //  Find the Bounding Box of the Physical (x,y) Coordinates
            if pos.x < left   { left   = pos.x; }
            if pos.y < top    { top    = pos.y; }
            if pos.x > right  { right  = pos.x; }
            if pos.y > bottom { bottom = pos.y; }
        }
    };
    if left < f64::MAX && top < f64::MAX &&
        right > f64::MIN && bottom > f64::MIN {  //  (x_virtual,y_virtual) found
            Some((left.floor(), top.floor(), right.floor(), bottom.floor())) 
    } else { None }  //  (x_virtual,y_virtual) not found
}
    
/// Given a normalised point, return the Physical (x,y) Coordinates
fn transform_physical_point(v: cg::Point2<f64>) -> cg::Point2<f64> {
    cg::Point2::new(
        v.x * X_PHYSICAL_SCALE - PHYSICAL_OFFSET.x,
        v.y * Y_PHYSICAL_SCALE - PHYSICAL_OFFSET.y
    )
    //  Previously: cg::Point2::from_vec((v * SCALE).to_vec() - GRID_OFFSET)
}

/// Given a normalised point, return the Virtual (x,y) Coordinates
fn transform_virtual_point(v: cg::Point2<f64>) -> cg::Point2<f64> {
    cg::Point2::new(
        v.x * X_VIRTUAL_SCALE - VIRTUAL_OFFSET.x,
        v.y * Y_VIRTUAL_SCALE - VIRTUAL_OFFSET.y
    )
}

fn get_normals(delaunay: &Delaunay) -> Vec<(na::Point3<f32>, na::Point3<f32>)> {
    let mut result = Vec::new();
    for v in delaunay.vertices() {
        let n = v.normal;
        let p = v.position_3d();
        result.push((cg_vec_to_na(p.to_vec()), cg_vec_to_na(p.to_vec() - n * 0.3)));
    }
    result
}

fn extract_edges(delaunay: &Delaunay) -> Vec<(na::Point3<f32>, na::Point3<f32>)> {
    let offset = cg::Vector3::new(0., 0., -0.01);
    let mut lines = Vec::new();
    for edge in delaunay.edges() {
        let from_pos = cg_vec_to_na(edge.from().position_3d().to_vec() + offset);
        let to_pos = cg_vec_to_na(edge.to().position_3d().to_vec() + offset);
        lines.push((from_pos, to_pos));
    }
    lines
}

fn create_mesh_from_triangulation(delaunay: &Delaunay) -> Mesh {
    let mut coords = Vec::new();
    let mut faces = Vec::new();
    for vertex in delaunay.vertices() {
        coords.push(cg_vec_to_na(vertex.position_3d().to_vec()));
    }
    for triangle in delaunay.triangles() {
        let triangle = triangle.as_triangle();
        let h0 = triangle[0].fix();
        let h1 = triangle[1].fix();
        let h2 = triangle[2].fix();
        faces.push(na::Point3::new(h0 as u16, h1 as u16, h2 as u16));
    }
    Mesh::new(coords, faces, None, None, false)
}
