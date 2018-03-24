use cgmath::prelude::*;
use cgmath::{Matrix3, Point3, vec3, Vector3, Rad};
use walker::Walker;

const MOVE_SPEED: f32 = 2.0;
const TURN_SPEED: f32 = 2.5;

pub struct Camera {
    pub pos: Point3<f32>,
    pub dir: Vector3<f32>,
    pub up: Vector3<f32>,
    pub upside_down: bool,
}

impl Camera {
    pub fn new(i: usize, j: usize, dir: Vector3<f32>) -> Camera {
        Camera {
            pos: Point3::new(i as f32 + 0.5, 0.0, j as f32 + 0.5),
            dir: dir,
            up: vec3(0.0, 1.0, 0.0),
            upside_down: false
        }
    }

    pub fn looking_at(&self, v_dir: Vector3<f32>) -> bool {
        self.dir.angle(v_dir) < Rad(0.01)
    }

    pub fn rotation_sign(&self, v1: &Vector3<f32>, v2: &Vector3<f32>) -> f32 {
        let sign = if v1.cross(*v2).y > 0.0 { 1.0 } else { -1.0 };
        if self.upside_down {
            -1.0 * sign
        } else {
            sign
        }
    }

    pub fn rotate_to(&mut self, v_dir: Vector3<f32>, dt: f32) -> bool {
        let sign = self.rotation_sign(&self.dir, &v_dir);

        self.dir = Matrix3::from_axis_angle(
            self.up, Rad(dt * sign * TURN_SPEED)) * self.dir;

        // if the rotation went through the objective, just assign it
        let new_sign = self.rotation_sign(&self.dir, &v_dir);
        if (sign * new_sign) < 0.0 {
            self.dir = v_dir;
        }

        self.looking_at(v_dir)
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

    pub fn roll_to(&mut self, v_dir: Vector3<f32>, dt: f32) -> bool {
        self.up = Matrix3::from_axis_angle(
            self.dir,
            Rad(dt * TURN_SPEED)) * self.up;

        if self.up.angle(v_dir) < Rad(0.1) {
            self.up = v_dir;
            true
        } else {
            false
        }
    }
}
