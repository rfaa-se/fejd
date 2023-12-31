use crate::math::{Flint, FlintRectangle, FlintTriangle, FlintVec2};

pub struct Miscellaneous {
    pub player_death_counters: Vec<(usize, Counter)>,
    pub player_map_spawn_indexes: Vec<usize>,
}

pub struct Counter {
    pub value: i32,
}

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

impl Miscellaneous {
    pub fn new() -> Self {
        Self {
            player_death_counters: Vec::new(),
            player_map_spawn_indexes: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.player_death_counters.clear();
        self.player_map_spawn_indexes.clear();
    }
}

impl Body<FlintRectangle> {
    pub fn new(shape: Shape<FlintRectangle>) -> Self {
        let mut body = Self {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        body.calc_axes(false);

        body
    }

    pub fn calc_axes(&mut self, include_past_body: bool) -> &[FlintVec2] {
        if self.dirty {
            self.axes.clear();

            let (sin, cos) = self.live.direction.sin_cos();
            let ox = self.live.shape.width / 2;
            let oy = self.live.shape.height / 2;
            let dx = -ox;
            let dy = -oy;
            let w = self.live.shape.width;
            let h = self.live.shape.height;
            let x = self.live.shape.point.x + ox;
            let y = self.live.shape.point.y + oy;

            if include_past_body {
                // we want to create 4 points, the 2 starting points should be the "end" of the past body,
                // and the 2 remaining points should be the end of the live body,
                // this is so we can collision check the entire path the rectangle has moved
                //   end ->  ._.  <- end
                //           |_|  <- live body
                //           . .
                //           . .  <- empty space
                //           . .
                // start ->  ._.  <- start
                //           |_|  <- past body

                let (psin, pcos) = self.past.direction.sin_cos();
                let pox = self.past.shape.width / 2;
                let poy = self.past.shape.height / 2;
                let pdx = -pox;
                let pdy = -poy;
                let pw = self.past.shape.width;
                let ph = self.past.shape.height;
                let px = self.past.shape.point.x + pox;
                let py = self.past.shape.point.y + poy;

                // top left
                self.axes.push(FlintVec2 {
                    x: px + (pdx + pw) * pcos - pdy * psin,
                    y: py + (pdx + pw) * psin + pdy * pcos,
                });

                // top right
                self.axes.push(FlintVec2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                });

                // bottom right
                self.axes.push(FlintVec2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                });

                // bottom left
                self.axes.push(FlintVec2 {
                    x: px + (pdx + pw) * pcos - (pdy + ph) * psin,
                    y: py + (pdx + pw) * psin + (pdy + ph) * pcos,
                });
            } else {
                // top left
                self.axes.push(FlintVec2 {
                    x: x + dx * cos - dy * sin,
                    y: y + dx * sin + dy * cos,
                });

                // top right
                self.axes.push(FlintVec2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                });

                // bottom right
                self.axes.push(FlintVec2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                });

                // bottom left
                self.axes.push(FlintVec2 {
                    x: x + dx * cos - (dy + h) * sin,
                    y: y + dx * sin + (dy + h) * cos,
                });
            }

            self.dirty = false;
        }

        &self.axes
    }
}

impl Body<FlintTriangle> {
    pub fn new(shape: Shape<FlintTriangle>) -> Self {
        let mut body = Self {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        body.calc_axes();

        body
    }

    pub fn calc_axes(&mut self) -> &[FlintVec2] {
        if self.dirty {
            self.axes.clear();

            let cen = self.live.shape.centroid();
            let rad = self.live.direction.radians();

            // TODO: v2, v1, v3, order matters here, why this order..?
            self.axes.append(&mut vec![
                self.live.shape.v2.rotated(rad, cen),
                self.live.shape.v1.rotated(rad, cen),
                self.live.shape.v3.rotated(rad, cen),
            ]);

            self.dirty = false;
        };

        &self.axes
    }
}
