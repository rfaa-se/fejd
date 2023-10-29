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

use crate::math::{Flint, FlintVec2};

pub fn project(shape_alpha: &[FlintVec2], axis: FlintVec2) -> FlintVec2 {
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

    // p1 = (4, 6);
    // p2 = (5, 5);
    // 6 > 5 || 4 > 5
    p1.y > p2.x || p1.x > p2.y
}

pub fn intersects(shape_alpha: &[FlintVec2], shape_beta: &[FlintVec2]) -> bool {
    // calculate projections /

    let mut axes1 = Vec::new();
    for i in 0..shape_alpha.len() {
        let edge = shape_alpha[i] - shape_alpha[if i + 1 == shape_alpha.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        axes1.push(perp);
    }

    for i in 0..axes1.len() {
        let axis = axes1[i];

        let p1 = project(shape_alpha, axis); // p1 = [FlintVec, FlintVec]
        let p2 = project(shape_beta, axis);

        if !overlap(p1, p2) {
            return false;
        }
    }

    // HIT
    //let proj_alpha = FlintVec2::new(min_alpha, max_alpha);

    let mut axes2 = Vec::new();
    for i in 0..shape_beta.len() {
        let edge = shape_beta[i] - shape_beta[if i + 1 == shape_beta.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        axes2.push(perp);
    }

    for i in 0..axes2.len() {
        let axis = axes2[i];

        let p1 = project(shape_alpha, axis); // p1 = [FlintVec, FlintVec]
        let p2 = project(shape_beta, axis);

        if !overlap(p1, p2) {
            return false;
        }
    }

    true
}
