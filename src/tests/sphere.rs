use cgmath::sphere::*;
use cgmath::point::*;
use cgmath::vector::*;
use cgmath::ray::*;
use cgmath::intersect::Intersect;
use std::num;

#[test]
fn test_intersection() {
    let sphere = Sphere {center: Point3::new(0f64,0f64,0f64), radius: 1f64};
    let r0: Ray3<f64> = Ray::new(Point3::new(0f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r1: Ray3<f64> = Ray::new(Point3::new(num::cos(1f64), 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r2: Ray3<f64> = Ray::new(Point3::new(1f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r3: Ray3<f64> = Ray::new(Point3::new(2f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    assert_eq!((sphere,r0).intersection(), Some(Point3::new(0f64, 0f64, 1f64)));
    assert_approx_eq!((sphere,r1).intersection().unwrap(), Point3::new(num::cos(1f64), 0f64, num::sin(1f64)));
    assert_eq!((sphere,r2).intersection(), Some(Point3::new(1f64, 0f64, 0f64)));
    assert_eq!((sphere,r3).intersection(), None);
}