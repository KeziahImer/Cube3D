use std::f32::consts::PI;
use crate::terminal::Terminal;

const CAMERA_DISTANCE: f32 = 2.1;
const X_RATIO: f32 = 20.0;
const Y_RATIO: f32 = 10.0;

struct Point2D {
    x: f32,
    y: f32,
}

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Cube {
    vertices_2d: Vec<Point2D>,
    vertices_3d: Vec<Point3D>,
    edges: Vec<(usize, usize)>,
    rotation: Rotation,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            vertices_2d: vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
            ],
            vertices_3d: vec![
                Point3D { x: -1.0, y: -1.0, z: -1.0 },
                Point3D { x:  1.0, y: -1.0, z: -1.0 },
                Point3D { x:  1.0, y:  1.0, z: -1.0 },
                Point3D { x: -1.0, y:  1.0, z: -1.0 },
                Point3D { x: -1.0, y: -1.0, z:  1.0 },
                Point3D { x:  1.0, y: -1.0, z:  1.0 },
                Point3D { x:  1.0, y:  1.0, z:  1.0 },
                Point3D { x: -1.0, y:  1.0, z:  1.0 },
            ],
            edges: vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
            ],
            rotation: Rotation {
                x: PI / 180.0,
                y: PI / 360.0,
                z: 0.0,
            }
        }
    }

    pub fn update(&mut self) {
        self.rotate();
        self.project();
    }

    pub fn render(&mut self, terminal: &Terminal) {
        terminal.clear();
        self.draw_vertices(terminal);
        self.draw_edges(terminal);
        terminal.flush();
    }

    fn rotate(&mut self) {
        self.rotate_x();
        self.rotate_y();
        self.rotate_z();
    }

    fn rotate_x(&mut self) {
        for vertex in &mut self.vertices_3d {
            let y = vertex.y;
            let z = vertex.z;
            vertex.y = y * self.rotation.x.cos() - z * self.rotation.x.sin();
            vertex.z = y * self.rotation.x.sin() + z * self.rotation.x.cos();
        }
    }

    fn rotate_y(&mut self) {
        for vertex in &mut self.vertices_3d {
            let x = vertex.x;
            let z = vertex.z;
            vertex.x = x * self.rotation.y.cos() + z * self.rotation.y.sin();
            vertex.z = z * self.rotation.y.cos() - x * self.rotation.y.sin();
        }
    }

    fn rotate_z(&mut self) {
        for vertex in &mut self.vertices_3d {
            let x = vertex.x;
            let y = vertex.y;
            vertex.x = x * self.rotation.z.cos() - y * self.rotation.z.sin();
            vertex.y = x * self.rotation.z.sin() + y * self.rotation.z.cos();
        }
    }

    fn project(&mut self) {
        for (i, vertex) in self.vertices_3d.iter().enumerate() {
            let z = vertex.z;
            let x = vertex.x / (z + CAMERA_DISTANCE) * X_RATIO;
            let y = vertex.y / (z + CAMERA_DISTANCE) * Y_RATIO;

            self.vertices_2d[i] = Point2D { x, y };
        }
    }

    fn draw_vertices(&self, terminal: &Terminal) {
        for (i, vertex) in self.vertices_2d.iter().enumerate() {
            terminal.draw(vertex.x, vertex.y, self.vertices_3d[i].z);
        }
    }

    fn draw_edges(&self, terminal: &Terminal) {
        self.draw_edge(terminal, self.edges[0].0, self.edges[0].1);
        self.draw_edge(terminal, self.edges[1].0, self.edges[1].1);
        self.draw_edge(terminal, self.edges[2].0, self.edges[2].1);
        self.draw_edge(terminal, self.edges[3].0, self.edges[3].1);
        self.draw_edge(terminal, self.edges[4].0, self.edges[4].1);
        self.draw_edge(terminal, self.edges[5].0, self.edges[5].1);
        self.draw_edge(terminal, self.edges[6].0, self.edges[6].1);
        self.draw_edge(terminal, self.edges[7].0, self.edges[7].1);
        self.draw_edge(terminal, self.edges[8].0, self.edges[8].1);
        self.draw_edge(terminal, self.edges[9].0, self.edges[9].1);
        self.draw_edge(terminal, self.edges[10].0, self.edges[10].1);
        self.draw_edge(terminal, self.edges[11].0, self.edges[11].1);
    }

    fn draw_edge(&self, terminal: &Terminal, i: usize, j: usize) {
        let mut x1 = self.vertices_2d[i].x;
        let mut y1 = self.vertices_2d[i].y;
        let x2 = self.vertices_2d[j].x;
        let y2 = self.vertices_2d[j].y;

        // let m = (y2 - y1) / (x2 - x1);

        // if m > 1.0 {
        //     let temp = x1;
        //     x1 = y1;
        //     y1 = temp;
        //     let temp = x2;
        //     x2 = y2;
        //     y2 = temp;
        // }
        // if x1 > x2 {
        //     let temp = x1;
        //     x1 = x2;
        //     x2 = temp;
        //     let temp = y1;
        //     y1 = y2;
        //     y2 = temp;
        // }

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            if x1 == x2 && y1 == y2 || sx == 1 && x1 > x2  || sx == -1 && x1 < x2 || sy == 1 && y1 > y2 || sy == -1 && y1 < y2 {
                break;
            }

            let e2 = 2.0 * err;

            if e2 > -dy {
                err -= dy;
                x1 += sx as f32;
            }
            if e2 < dx {
                err += dx;
                y1 += sy as f32;
            }
            terminal.draw(x1, y1, 0.0);
        }
    }

}
