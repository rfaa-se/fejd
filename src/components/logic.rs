use crate::math::{Flint, FlintRectangle, FlintTriangle, FlintVec2};

pub struct Body<T> {
    pub shape: T,
    pub rotation: FlintVec2,
    pub dirty: bool,
    pub axes: Vec<FlintVec2>,
}

pub struct Motion {
    pub speed: Flint,
    pub max_speed: Flint,
    pub acceleration: Flint,
    pub rotation_speed: Flint,
}

impl Body<FlintRectangle> {
    pub fn get_axes(&mut self) -> &Vec<FlintVec2> {
        if self.dirty {
            // TODO: räkna om axes
            self.axes.clear();

            self.axes.append(&mut vec![
                FlintVec2 {
                    x: self.shape.point.x,
                    y: self.shape.point.y,
                }
                .rotate(&self.rotation.radians(), &self.shape.get_centroid()),
                FlintVec2 {
                    x: self.shape.point.x + self.shape.width,
                    y: self.shape.point.y,
                },
                FlintVec2 {
                    x: self.shape.point.x + self.shape.width,
                    y: self.shape.point.y + self.shape.height,
                },
                FlintVec2 {
                    x: self.shape.point.x,
                    y: self.shape.point.y + self.shape.height,
                },
            ]);
        }

        &self.axes
    }
}

impl Body<FlintTriangle> {
    pub fn get_axes(&mut self) -> &Vec<FlintVec2> {
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
