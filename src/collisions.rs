use crate::math::{Flint, FlintVec2};

fn project(shape_alpha: &[FlintVec2], axis: &FlintVec2) -> FlintVec2 {
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

fn is_overlapping(p1: FlintVec2, p2: FlintVec2) -> bool {
    p1.y > p2.x || p1.x > p2.y
}

// fn calc_overlap(p1: FlintVec2, p2: FlintVec2) -> Flint {
//     if p1.y > p2.x {
//         p1.y - p2.x
//     } else {
//         p2.x - p1.y
//     }
// }

fn calc_perps(shape: &[FlintVec2]) -> Vec<FlintVec2> {
    let mut perps = Vec::new();

    for i in 0..shape.len() {
        let edge = shape[i] - shape[if i + 1 == shape.len() { 0 } else { i + 1 }];
        let perp = edge.perpendicular();
        let perp = perp.normalized(); // TODO: needed?
        perps.push(perp);
    }

    perps
}

pub fn intersects(shape_alpha: &[FlintVec2], shape_beta: &[FlintVec2]) -> bool {
    let axes = calc_perps(shape_alpha);
    for axis in axes.iter() {
        let p1 = project(shape_alpha, axis);
        let p2 = project(shape_beta, axis);

        if !is_overlapping(p1, p2) {
            return false;
        }
    }

    let axes = calc_perps(shape_beta);
    for axis in axes.iter() {
        let p1 = project(shape_alpha, axis);
        let p2 = project(shape_beta, axis);

        if !is_overlapping(p1, p2) {
            return false;
        }
    }

    true
}

pub fn calculate_speed_to_collision(
    direction: FlintVec2,
    // speed: Flint,
    shape_alpha: &[FlintVec2],
    shape_beta: &[FlintVec2],
) -> Flint {
    let mut alpha = vec![FlintVec2::new(Flint::ZERO, Flint::ZERO); shape_alpha.len()];
    alpha.copy_from_slice(shape_alpha);

    // let threshold = Flint::from_num(0.2);
    let mut total = Flint::ZERO;
    // halving speed is probably faster but there's a risk
    // of entirely missing the intersection, let's move
    // with a speed of one instead
    // let mut speed = speed / 2;
    let speed = Flint::ONE;

    // while speed > threshold {
    loop {
        let velocity = direction * speed;

        // apply velocity to alpha
        for a in alpha.iter_mut() {
            *a += velocity;
        }

        if intersects(&alpha, shape_beta) {
            // move back
            // for a in alpha.iter_mut() {
            //     *a -= velocity;
            // }

            break;
        } else {
            total += speed;
        }

        // speed /= 2;
    }

    total
}
