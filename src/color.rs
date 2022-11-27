use crate::vec3::Vector3;

pub type Color = Vector3;

pub fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * color.x) as i64,
        (255.999 * color.y) as i64,
        (255.999 * color.z) as i64,
    )
}
