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

pub fn intersects(axes_alpha: &[FlintVec2], axes_beta: &[FlintVec2]) -> bool {
    // calculate projections /

    let mut min_alpha = Flint::MAX;
    let mut max_alpha = Flint::MIN;
    let mut min_beta = Flint::MAX;
    let mut max_beta = Flint::MAX;

    for i in 0..axes_alpha.len() {
        let edge = axes_alpha[i] - axes_alpha[if i + 1 == axes_alpha.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        let dot = perp.dot(&axes_alpha[i]);

        if dot < min_alpha {
            min_alpha = dot;
        } else if dot > max_alpha {
            max_alpha = dot;
        }
    }
    let proj_alpha = FlintVec2::new(min_alpha, max_alpha);

    for i in 0..axes_beta.len() {
        let edge = axes_beta[i] - axes_beta[if i + 1 == axes_beta.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        let dot = perp.dot(&axes_beta[i]);

        if dot < min_beta {
            min_beta = dot;
        } else if dot > max_beta {
            max_beta = dot;
        }
    }

    let proj_beta = FlintVec2::new(min_beta, max_beta);

    if min_alpha >= max_beta || min_beta >= max_alpha {
        return false;
    }

    true
}
