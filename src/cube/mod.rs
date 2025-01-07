use std::f32::consts::PI;
use crate::terminal::Terminal;

const CAMERA_DISTANCE: f32 = 2.0;

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
            rotation: Rotation {
                x: PI / 180.0,
                y: PI / 90.0,
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
        self.draw(terminal);
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
            let mut z = vertex.z;
            if z == 0.0 {
                z = 0.0001;
            }
            let x = vertex.x / (z + CAMERA_DISTANCE);
            let y = vertex.y / (z + CAMERA_DISTANCE);

            self.vertices_2d[i] = Point2D { x, y };
        }
    }

    fn draw(&self, terminal: &Terminal) {
        for (i, vertex) in self.vertices_2d.iter().enumerate() {
            terminal.draw(vertex.x, vertex.y, self.vertices_3d[i].z);
        }
    }
}
