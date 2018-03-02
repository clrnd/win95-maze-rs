use cgmath::prelude::*;
use cgmath::{Matrix3, Point3, vec3, Vector3, Rad};
use walker::Walker;

const MOVE_SPEED: f32 = 2.0;
const TURN_SPEED: f32 = 2.5;

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

    pub fn rotation_sign(v1: &Vector3<f32>, v2: &Vector3<f32>) -> f32 {
        if v1.cross(*v2).y > 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn rotate_to(&mut self, v_dir: Vector3<f32>, dt: f32) {
        let sign = Camera::rotation_sign(&self.dir, &v_dir);

        self.dir = Matrix3::from_angle_y(Rad(dt * sign * TURN_SPEED)) * self.dir;

        // if the rotation went through the objective, just assign it
        let new_sign = Camera::rotation_sign(&self.dir, &v_dir);;
        if (sign * new_sign) < 0.0 {
            self.dir = v_dir;
        }
    }

    pub fn move_to(&mut self, p_to: Point3<f32>, dt: f32) -> bool {
        let old_dir = (p_to - self.pos).normalize();

        self.pos += MOVE_SPEED * dt * self.dir;

        // if new_dir is opposite direction from old_dir
        // then we went through, just assign it
        let new_dir = (p_to - self.pos).normalize();
        if old_dir.distance(new_dir) < 0.5 {
            false
        } else {
            self.pos = p_to;
            true
        }
    }
}
