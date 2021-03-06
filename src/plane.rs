//   Copyright Colin Sherratt 2014
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use super::Quad;
use super::generators::{SharedVertex, IndexedPolygon};

/// Represents a 2D plane with origin of (0, 0), from 1 to -1
#[derive(Copy)]
pub struct Plane {
    subdivide_x: usize,
    subdivide_y: usize,
    x: usize,
    y: usize
}

impl Plane {
    /// create a new cube generator
    pub fn new() -> Plane {
        Plane {
            subdivide_x: 1,
            subdivide_y: 1,
            x: 0,
            y: 0
        }
    }

    /// create a subdivided plane. This can be used to build
    /// a grid of points.
    /// x is the number of subdivisions in the x axis
    /// y is the number of subdivisions in the y axis
    pub fn subdivide(x: usize, y: usize) -> Plane {
        assert!(x > 0 && y > 0);
        Plane {
            subdivide_x: x,
            subdivide_y: y,
            x: 0,
            y: 0
        }
    }

    fn vert(&self, x: usize, y: usize) -> (f32, f32) {
        let sx = self.subdivide_x as f32;
        let sy = self.subdivide_y as f32;
        let x = (2. / sx) * x as f32 - 1.;
        let y = (2. / sy) * y as f32 - 1.;
        (x, y)
    }
}

impl Iterator for Plane {
    type Item = Quad<(f32, f32)>;

    fn next(&mut self) -> Option<Quad<(f32, f32)>> {
        if self.x == self.subdivide_x {
            self.x = 0;
            self.y += 1;
            if self.y == self.subdivide_y {
                return None;
            }
        }

        let x = self.vert(self.x,   self.y);
        let y = self.vert(self.x+1, self.y);
        let z = self.vert(self.x+1, self.y+1);
        let w = self.vert(self.x,   self.y+1);
        self.x += 1;

        Some(Quad::new(x, y, z, w))
    }
}

impl SharedVertex<(f32, f32)> for Plane {
    fn shared_vertex(&self, idx: usize) -> (f32, f32) {
        let y = idx / (self.subdivide_x + 1);
        let x = idx % (self.subdivide_x + 1);

        self.vert(x, y)
    }

    fn shared_vertex_count(&self) -> usize {
        (self.subdivide_x + 1) * (self.subdivide_y + 1)
    }
}

impl IndexedPolygon<Quad<usize>> for Plane {
    fn indexed_polygon(&self, idx: usize) -> Quad<usize> {
        let y = idx / (self.subdivide_x);
        let y = y * (self.subdivide_x+1);
        let x = idx % self.subdivide_x;

        Quad::new((x+y) + self.subdivide_x + 1,
                  (x+y),
                  (x+y) + 1,
                  (x+y) + self.subdivide_x + 2)
    }

    fn indexed_polygon_count(&self) -> usize {
        self.subdivide_x * self.subdivide_y
    }
}

#[test]
fn test_shared_vertex_count() {
    let plane = Plane::new();
    assert_eq!(plane.shared_vertex_count(), 4);
    assert_eq!(plane.indexed_polygon_count(), 1);

    let plane = Plane::subdivide(2, 2);
    assert_eq!(plane.shared_vertex_count(), 9);
    assert_eq!(plane.indexed_polygon_count(), 4);


    let plane = Plane::subdivide(4, 4);
    assert_eq!(plane.shared_vertex_count(), 25);
    assert_eq!(plane.indexed_polygon_count(), 16);
}

