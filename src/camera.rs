use cgmath::{Matrix3, InnerSpace, Point3, vec3, Vector3,
             Rad, EuclideanSpace, MetricSpace};
use walker::Walker;

const MOVE_SPEED: f32 = 3.0;
const TURN_SPEED: f32 = 3.0;

pub struct Camera {
    pub pos: Point3<f32>,
    pub dir: Vector3<f32>,
    pub up: Vector3<f32>
}

impl Camera {
    pub fn new(walker: &Walker) -> Camera {
        Camera {
            pos: Point3::new(walker.i as f32 + 0.5, 0.0, walker.j as f32 + 0.5),
            dir: walker.direction.to_vec(),
            up: vec3(0.0, 1.0, 0.0)
        }
    }

    pub fn looking_at(&self, v_dir: Vector3<f32>) -> bool {
        self.dir.angle(v_dir) < Rad(0.01)
    }

    pub fn rotate_to(&mut self, v_dir: Vector3<f32>, dt: f32) {
        let sign = if self.dir.cross(v_dir).y > 0.0 { 1.0 } else { -1.0 };
        self.dir = Matrix3::from_angle_y(Rad(dt * sign * TURN_SPEED)) * self.dir;

        // if the rotation went trough the objective, just assign it
        let new_sign = if self.dir.cross(v_dir).y > 0.0 { 1.0 } else { -1.0 };
        if (sign * new_sign) < 0.0 {
            self.dir = v_dir;
        }
    }

    pub fn move_to(&mut self, v_to: Vector3<f32>, dt: f32) -> bool {
        self.pos += MOVE_SPEED * dt * self.dir;

        self.pos.to_vec().distance(v_to) < 0.1
    }
}
