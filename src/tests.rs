use crate::grfx::vector::Vec3D;
#[test]
fn vector_dot_product() {
    let lhs = Vec3D::new(1.0,2.0,3.0);
    let rhs = Vec3D::new(1.0,2.0,3.0);
    assert_eq!(Vec3D::dot(&lhs, &rhs), 14.0);
}
#[test]
fn vector_cross_product () {
    let lhs = Vec3D::new(1.0,2.0,3.0);
    let rhs = Vec3D::new(3.0,4.0,5.0);
    assert_eq!(Vec3D::cross(&lhs, &rhs), Vec3D::new(-2.0, 4.0, -2.0));
}
#[test]
fn vector_cross_product_zero () {
    let lhs = Vec3D::new(1.0,2.0,3.0);
    let rhs = Vec3D::new(1.0,2.0,3.0);
    assert_eq!(Vec3D::cross(&lhs, &rhs), Vec3D::new(0.0, 0.0, 0.0));
}

#[test]
fn vector_multiply() {
    let lhs = Vec3D::new(1.0,2.0,3.0);
    let rhs = 2.0;
    assert_eq!(lhs * rhs, Vec3D::new(2.0, 4.0, 6.0));
}

#[test]
fn vector_divide() {
    let lhs = Vec3D::new(2.0,4.0,6.0);
    let rhs = 2.0;
    assert_eq!(lhs / rhs, Vec3D::new(1.0,2.0,3.0));
}

#[test]
fn vector_unit() {
    let lhs = Vec3D::new(5.0,5.0,5.0);
    assert_eq!(lhs.unit_vector(), Vec3D::new(1.0/3.0_f32.sqrt(),1.0/3.0_f32.sqrt(),1.0/3.0_f32.sqrt()));
}
