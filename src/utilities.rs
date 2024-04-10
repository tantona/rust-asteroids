use raylib::prelude::*;

pub fn rotate_point(p1: Vector2, mut p2: Vector2, angle: f32) -> Vector2 {
    let s = angle.sin();
    let c = angle.cos();

    p2.x -= p1.x;
    p2.y -= p1.y;

    let xnew = p2.x * c - p2.y * s;
    let ynew = p2.x * s + p2.y * c;

    p2.x = xnew + p1.x;
    p2.y = ynew + p1.y;

    p2
}

pub fn centroid_of_triangle(p1: Vector2, p2: Vector2, p3: Vector2) -> Vector2 {
    Vector2::new((p1.x + p2.x + p3.x) / 3.0, (p1.y + p2.y + p3.y) / 3.0)
}
