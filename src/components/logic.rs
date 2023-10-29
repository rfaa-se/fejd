use crate::math::{Flint, FlintRectangle, FlintTriangle, FlintVec2};

pub struct Body<T> {
    pub live: Shape<T>,
    pub past: Shape<T>,
    pub dirty: bool,
    pub axes: Vec<FlintVec2>,
}

#[derive(Clone, Copy)]
pub struct Shape<T> {
    pub shape: T,
    pub rotation: FlintVec2,
}

pub struct Motion {
    pub speed: Flint,
    pub max_speed: Flint,
    pub acceleration: Flint,
    pub rotation_speed: Flint,
}

impl Body<FlintRectangle> {
    pub fn get_axes(&mut self, include: bool) -> &Vec<FlintVec2> {
        if self.dirty {
            self.axes.clear();

            let rec = if include {
                let dx = (self.live.shape.point.x - self.past.shape.point.x).abs();
                let x = self.past.shape.point.x;

                let dy = (self.live.shape.point.y - self.past.shape.point.y).abs();
                let y = self.past.shape.point.y;

                let oc = self.past.shape.get_centroid();
                let dc = FlintVec2 {
                    x: (self.live.shape.point.x - oc.x).abs(),
                    y: (self.live.shape.point.y - oc.y).abs(),
                };
                let c = oc + (dc / Flint::from_num(2));
                let w = ((x + dx) - x).abs() + self.live.shape.width;
                let h = ((y + dy) - y).abs() + self.live.shape.height;
                FlintRectangle::from_centroid(&c, w, h)
            } else {
                self.live.shape
            };

            self.axes.append(&mut vec![
                FlintVec2 {
                    x: rec.point.x,
                    y: rec.point.y,
                }
                .rotate(&self.live.rotation.radians(), &rec.get_centroid()),
                FlintVec2 {
                    x: rec.point.x + rec.width,
                    y: rec.point.y,
                }
                .rotate(&self.live.rotation.radians(), &rec.get_centroid()),
                FlintVec2 {
                    x: rec.point.x + rec.width,
                    y: rec.point.y + rec.height,
                }
                .rotate(&self.live.rotation.radians(), &rec.get_centroid()),
                FlintVec2 {
                    x: rec.point.x,
                    y: rec.point.y + rec.height,
                }
                .rotate(&self.live.rotation.radians(), &rec.get_centroid()),
                // FlintVec2 { x, y }.rotate(
                //     &self.live.rotation.radians(),
                //     &self.live.shape.get_centroid(),
                // ),
                // FlintVec2 {
                //     x: self.live.shape.point.x + self.live.shape.width,
                //     y: self.live.shape.point.y,
                // }
                // .rotate(
                //     &self.live.rotation.radians(),
                //     &self.live.shape.get_centroid(),
                // ),
                // FlintVec2 {
                //     x: self.live.shape.point.x + self.live.shape.width,
                //     y: self.live.shape.point.y + self.live.shape.height,
                // }
                // .rotate(
                //     &self.live.rotation.radians(),
                //     &self.live.shape.get_centroid(),
                // ),
                // FlintVec2 {
                //     x: self.live.shape.point.x,
                //     y: self.live.shape.point.y + self.live.shape.height,
                // }
                // .rotate(
                //     &self.live.rotation.radians(),
                //     &self.live.shape.get_centroid(),
                // ),
            ]);

            self.dirty = false;
        }

        &self.axes
    }
}

impl Body<FlintTriangle> {
    pub fn get_axes(&mut self) -> &Vec<FlintVec2> {
        if self.dirty {
            // TODO: past body?!
            self.axes.clear();
            self.axes.append(&mut vec![
                self.live.shape.v1.rotate(
                    &self.live.rotation.radians(),
                    &self.live.shape.get_centroid(),
                ),
                self.live.shape.v2.rotate(
                    &self.live.rotation.radians(),
                    &self.live.shape.get_centroid(),
                ),
                self.live.shape.v3.rotate(
                    &self.live.rotation.radians(),
                    &self.live.shape.get_centroid(),
                ),
            ]);
            self.dirty = false;
        };

        &self.axes
    }
}

// impl<T: BodyAxesCalculation> Body<T> {
//     pub fn get_axes(&mut self) -> &Vec<FlintVec2> {
//         if self.dirty {
//             //self.calculate_axes();
//             self.shape.calculate_axes(&self.rotation);
//             self.dirty = false;
//         }

//         &self.axes
//     }
// }

// impl BodyAxesCalculation for Body<FlintTriangle> {
//     fn calculate_axes(&mut self, rotation: &FlintVec2) {
//         self.axes.clear();
//         self.axes.push(
//             self.shape
//                 .v1
//                 .rotate(self.rotation, &self.shape.get_centroid()),
//         );
//     }
// }

// impl<T: BodyAxesGet + BodyCentroidGet> BodyAxesCalculation for Body<T> {
//     fn calculate_axes(&mut self) {
//         let axes = self.get_axes();
//         for axis in axes {
//             // TODO: get_centroid????
//             let rotated_axis = axis.rotate(&self.rotation.radians(), &self.shape.get_centroid());
//             self.axes.push(rotated_axis);
//         }
//     }
// }

// impl<T: BodyAxesGet + BodyCentroidGet> Body<T> {
//     pub fn get_axes(&mut self) -> &Vec<FlintVec2> {
//         if self.dirty {
//             self.calculate_axes();
//             self.dirty = false;
//         }

//         &self.axes
//     }
// }

// pub trait BodyAxesCalculation {
//     fn calculate_axes(&mut self);
// }

// pub trait BodyAxesGet {
//     fn get_axes(&self) -> &Vec<FlintVec2>;
// }

// pub trait BodyCentroidGet {
//     fn get_centroid(&self) -> FlintVec2;
// }
