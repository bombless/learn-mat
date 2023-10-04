use glam::{Mat4, Quat, Vec3};
fn main() {
    let ratio = 16.0 / 9.0;
    let q = Quat::from_scaled_axis(Vec3::X * (1.0 / ratio));
    let mat = Mat4::from_scale(q.to_scaled_axis());
    let proj = mat.to_cols_array_2d();
    let array = [
        [1.0 / ratio, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(array, proj);
}
