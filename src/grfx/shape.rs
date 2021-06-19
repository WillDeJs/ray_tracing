use crate::grfx::color::Color;
use crate::grfx::material::Lambertian;
use crate::grfx::material::Material;
use crate::grfx::ray::Ray;
use crate::grfx::vector::Vec3D;
use std::rc::Rc;

/// Hit records which contains the point being hit and the constant hitting it as well as its normal
#[allow(dead_code)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3D,
    pub normal: Vec3D,
    pub material: Rc<dyn Material>,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Vec3D::new(0.0, 0.0, 0.0),
            normal: Vec3D::new(0.0, 0.0, 0.0),
            material: Rc::new(Lambertian::new(Color::rgb(255, 255, 255))),
        }
    }
}

/// Sphere structure
///     center
///     radius
/// Implements Hitable so that we can determine when the sphere is hit by rays
pub struct Sphere {
    center: Vec3D,
    radius: f32,
    material: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3D, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    pub fn center(&self) -> &Vec3D {
        &self.center
    }
    pub fn raidus(&self) -> f32 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32, record: &mut HitRecord) -> bool {
        // get components in the form of ax^2 + bx + c
        // Equation for a sphere is x^2 + y^2 + z^2 -R^2 = 0 -> x*x + y*y + z*z = R*R
        // If sphere center is recepresented by (cx, cy, cz) then
        // (x-cx)*(x-cx)  + (y-cy)*(y-cy) + (z-cz)*(z-cz)
        // this is the same as saying dot product since each element is multiplied by its corresponding match
        // dot[(point - center), (point-center)]
        // (point-center) . (point - center)
        // the point is given but the point/direction according to the ray theory is A + tB (origin + constant * point) and the center of the sphere we know
        // this becomes (A + tB- C) . (A + tB - C) = R*R
        // we can simplify:
        //              ((A-C) + tB). ((A-C) + tB)
        //              (A-C).(A-C) + 2*tB.(A-C)  + t*t*B.B = R*R
        // re-ordering gives
        // t*tB.B + 2*tB.(A-C) + (A-C).(A-C) which looks an awful lot like a polynomial of the form ax^2 + bx + c
        // we make it all = 0 and solve for the coefficients using quadratic formula
        //  t*tB.B + 2*tB.(A-C) + (A-C).(A-C) - R.R = 0
        // a = B.B
        // b = 2B.(A-C)
        // c = (A-C).(A-C) - R.R
        let origin_center = ray.origin() - self.center; // this A-C on the equauation above
        let a = Vec3D::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vec3D::dot(&ray.direction(), &origin_center);
        let c = Vec3D::dot(&origin_center, &origin_center) - self.radius * self.radius;

        // let's calculate the zeros for this equation
        let discriminant = b * b - 4.0 * a * c; // b^2 - 4ac under the sqrt()

        // got a real hit
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            // our t1 calculated value is within range?
            if t1 < max && t1 > min {
                record.t = t1;
                record.point = ray.point_at(t1);
                record.normal = (record.point - self.center) / self.radius;
                record.material = self.material.clone();
                return true;
            }

            // our t2 calculated value is within range?
            if t2 < max && t2 > min {
                record.t = t2;
                record.point = ray.point_at(t2);
                record.normal = (record.point - self.center) / self.radius;
                record.material = self.material.clone();
                return true;
            }
        }
        false
    }
}
/// Interface to be used by all hittable structures
/// The trait function hit shoudl be implemented by all shapes.
#[allow(dead_code)]
pub trait Hitable {
    fn hit(&self, ray: &Ray, min: f32, max: f32, record: &mut HitRecord) -> bool;
}

/// Array like structure that contains all hittable objects shown
/// It implements hittable in order to check all objects being hit at once
#[allow(dead_code)]
pub struct HitList {
    hitable_items: Vec<Rc<dyn Hitable>>,
}
impl HitList {
    pub fn new() -> Self {
        Self {
            hitable_items: Vec::new(),
        }
    }
    pub fn add(&mut self, item: Rc<dyn Hitable>) {
        &self.hitable_items.push(item);
    }
    pub fn get(&self, index: usize) -> Option<&Rc<dyn Hitable>> {
        self.hitable_items.get(index)
    }
}
impl Hitable for HitList {
    fn hit(&self, ray: &Ray, min: f32, max: f32, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut closest_so_far = max;
        let mut hit_anything = false;
        for ray_hit in &self.hitable_items {
            if ray_hit.hit(ray, min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                record.point = temp_record.point;
                record.t = temp_record.t;
                record.normal = temp_record.normal;
                record.material = temp_record.material.clone();
            }
        }
        hit_anything
    }
}
