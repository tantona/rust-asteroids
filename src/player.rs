use crate::utilities;
use raylib::prelude::*;

pub struct Player {
    window_width: f32,
    window_height: f32,
    position: Vector2,
    orientation: f32,
    speed: f32,
    velocity: Vector2,
    bullets: Vec<Bullet>,
}

impl Player {
    pub fn new(w: f32, h: f32) -> Self {
        return Player {
            window_width: w,
            window_height: h,
            position: Vector2::new(w / 2.0, h / 2.0),
            orientation: 0.0,
            speed: 5.0,
            velocity: Vector2::new(0.0, 0.0),
            bullets: Vec::new(),
        };
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            self.orientation -= 0.1;
        }

        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.orientation += 0.1;
        }

        if d.is_key_down(KeyboardKey::KEY_UP) {
            let ft = d.get_frame_time();
            let thrust = self.get_thrust();
            self.velocity += Vector2::new(thrust.x * ft, thrust.y * ft)
        }

        if d.is_key_pressed(KeyboardKey::KEY_DOWN) {}

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.bullets
                .push(Bullet::new(self.position.clone(), self.orientation.clone()));
        }

        self.position += self.velocity;

        self.cull_bullets();
    }

    fn cull_bullets(&mut self) {
        self.bullets = self
            .bullets
            .iter()
            .cloned()
            .filter(|b| {
                b.position.x < self.window_width
                    && b.position.x > 0.0
                    && b.position.y < self.window_height
                    && b.position.y > 0.0
            })
            .collect();

        println!("{:?}", self.bullets.len());
    }

    fn get_thrust(&self) -> Vector2 {
        Vector2::new(
            self.orientation.cos() * self.speed,
            self.orientation.sin() * self.speed,
        )
    }

    fn rotate(&mut self) -> Vec<Vector2> {
        let mut p1 = Vector2::new(self.position.x + 15.0, self.position.y);
        let mut p2 = Vector2::new(self.position.x - 10.0, self.position.y - 10.0);
        let mut p3 = Vector2::new(self.position.x - 10.0, self.position.y + 10.0);

        let centroid = self.position;
        p1 = utilities::rotate_point(centroid, p1, self.orientation);
        p2 = utilities::rotate_point(centroid, p2, self.orientation);
        p3 = utilities::rotate_point(centroid, p3, self.orientation);

        vec![p1, p2, p3]
    }

    fn wrap_position(&mut self) {
        if self.position.x >= self.window_width {
            self.position.x = 0.0;
        }

        if self.position.y >= self.window_height {
            self.position.y = 0.0;
        }

        if self.position.x < 0.0 {
            self.position.x = self.window_width;
        }

        if self.position.y < 0.0 {
            self.position.y = self.window_height
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.wrap_position();

        let points = self.rotate();

        d.draw_triangle_lines(points[0], points[1], points[2], Color::RAYWHITE);
        d.draw_circle_v(self.position, 1.5, Color::RED);

        for bullet in self.bullets.iter_mut() {
            bullet.draw(d)
        }
    }
}

#[derive(Clone, Debug)]
struct Bullet {
    position: Vector2,
    angle: f32,
    speed: f32,
}

impl Bullet {
    pub fn new(position: Vector2, angle: f32) -> Self {
        return Bullet {
            position,
            angle,
            speed: 15.0,
        };
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.position += Vector2::new(self.angle.cos() * self.speed, self.angle.sin() * self.speed);

        d.draw_circle_v(self.position, 2.5, Color::WHITE);
    }
}

// fn calculat_new_xy(p1: Vector2, speed: f32, angle_rad: f32) -> Vector2 {
//     let new_x = p1.x + (speed * angle_rad.cos());
//     let new_y = p1.y + (speed * angle_rad.sin());
//     Vector2::new(new_x, new_y)
// }
