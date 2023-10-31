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
    pub direction: FlintVec2,
}

pub struct Motion {
    pub speed: Flint,
    pub max_speed: Flint,
    pub acceleration: Flint,
    pub rotation_speed: Flint,
}

impl Body<FlintRectangle> {
    pub fn calc_axes(&mut self, include_past_body: bool) -> &Vec<FlintVec2> {
        if self.dirty {
            self.axes.clear();

            let (sin, cos) =
                cordic::sin_cos(cordic::atan2(self.live.direction.y, self.live.direction.x));
            let cx = self.live.shape.point.x + self.live.shape.width / 2;
            let cy = self.live.shape.point.y + self.live.shape.height / 2;

            let dx = self.live.shape.point.x - cx;
            let dy = self.live.shape.point.y - cy;

            if include_past_body {
                // we want to create 4 points, the 2 starting points should be the "end" of the past body,
                // and the 2 remaining points should be the end of the live body
                //   end ->  ._.  <- end
                //           |_|  <- live body
                //           . .
                //           . .  <- empty space
                //           . .
                // start ->  ._.  <- start
                //           |_|  <- past body
                let (psin, pcos) =
                    cordic::sin_cos(cordic::atan2(self.past.direction.y, self.past.direction.x));
                let pcx = self.past.shape.point.x + self.past.shape.width / 2;
                let pcy = self.past.shape.point.y + self.past.shape.height / 2;
                let pdx = self.past.shape.point.x - pcx;
                let pdy = self.past.shape.point.y - pcy;

                // top left
                self.axes.push(FlintVec2 {
                    x: pdx * pcos - pdy * psin + pcx,
                    y: pdx * psin + pdy * pcos + pcy,
                });

                let pdx = self.past.shape.point.x + self.past.shape.width - pcx;

                // top right
                self.axes.push(FlintVec2 {
                    x: pdx * pcos - pdy * psin + pcx,
                    y: pdx * psin + pdy * pcos + pcy,
                });

                let dy = self.live.shape.point.y + self.live.shape.height - cy;

                // bottom left
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });

                let dx = self.live.shape.point.x + self.live.shape.width - cx;

                // bottom right
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });

                // self.axes.swap(0, 3);
                // self.axes.swap(2, 1);
            } else {
                // top left
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });

                let dx = self.live.shape.point.x + self.live.shape.width - cx;

                // top right
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });

                let dy = self.live.shape.point.y + self.live.shape.height - cy;

                // bottom right
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });

                let dx = self.live.shape.point.x - cx;

                // bottom left
                self.axes.push(FlintVec2 {
                    x: dx * cos - dy * sin + cx,
                    y: dx * sin + dy * cos + cy,
                });
            }

            self.dirty = false;
        }

        &self.axes
    }
}

impl Body<FlintTriangle> {
    pub fn calc_axes(&mut self) -> &Vec<FlintVec2> {
        if self.dirty {
            self.axes.clear();

            let centroid = self.live.shape.centroid();
            let radians = self.live.direction.radians();

            // TODO: v2, v1, v3, order matters here, why this order..?
            self.axes.append(&mut vec![
                self.live.shape.v2.rotated(radians, centroid),
                self.live.shape.v1.rotated(radians, centroid),
                self.live.shape.v3.rotated(radians, centroid),
            ]);

            self.dirty = false;
        };

        &self.axes
    }
}
