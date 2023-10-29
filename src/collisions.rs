// use crate::{
//     components::logic::{Body, BodyAxesGet, BodyCentroidGet},
//     math::{FlintRectangle, FlintTriangle, FlintVec2},
// };

// pub trait Intersectable<T> {
//     fn intersects(&mut self, body: &mut Body<T>) -> bool;
// }

// // RECTANGLE

// impl<T> Body<T> {
//     fn intersects(&mut self, body: &mut T) -> bool
//     where
//         T: BodyAxesGet + BodyCentroidGet,
//     {
//         let axes_self = self.get_axes();
//         let axes_body = body.get_axes();

//         intersects(axes_self, axes_body)
//     }
// }

// TRIANGLE

// impl Intersectable<FlintTriangle> for Body<FlintRectangle> {
//     fn intersects(&self, body: &Body<FlintTriangle>) -> bool {
//         body.intersects(self)
//     }
// }

// LINE

// VECTOR

// TODO: add intersectable for remaining math shapes

use crate::math::FlintVec2;

pub fn project(shape_alpha: &[FlintVec2], axis: &FlintVec2) -> FlintVec2 {
    // beratna
    let mut min = axis.dot(&shape_alpha[0]);
    let mut max = min;

    for i in 1..shape_alpha.len() {
        let p = axis.dot(&shape_alpha[i]);

        if p < min {
            min = p;
        } else if p > max {
            max = p;
        }
    }

    FlintVec2::new(min, max)
}

pub fn overlap(p1: FlintVec2, p2: FlintVec2) -> bool {
    // p1 = (0, 1);
    // p2 = (2, 3);
    // 1 > 2 || 0 > 3

    // p1 = (4, 6);
    // p2 = (5, 5);
    // 6 > 5 || 4 > 5
    p1.y > p2.x || p1.x > p2.y
}

fn get_perps(shape: &[FlintVec2]) -> Vec<FlintVec2> {
    let mut perps = Vec::new();

    for i in 0..shape.len() {
        let edge = shape[i] - shape[if i + 1 == shape.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        perps.push(perp);
    }

    perps
}

pub fn intersects(shape_alpha: &[FlintVec2], shape_beta: &[FlintVec2]) -> bool {
    let axes = get_perps(shape_alpha);
    for axis in axes.iter() {
        let p1 = project(shape_alpha, axis);
        let p2 = project(shape_beta, axis);

        if !overlap(p1, p2) {
            return false;
        }
    }

    let axes = get_perps(shape_beta);
    for axis in axes.iter() {
        let p1 = project(shape_alpha, axis);
        let p2 = project(shape_beta, axis);

        if !overlap(p1, p2) {
            return false;
        }
    }

    true
}
