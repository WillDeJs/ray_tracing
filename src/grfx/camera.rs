use crate::grfx::ray::Ray;
use crate::grfx::vector::Vec3D;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Camera {
    lower_left: Vec3D, // lowerleft
    horizontal: Vec3D,
    vertical: Vec3D,
    origin: Vec3D,
    lens_radius: f32,
    u: Vec3D,
    v: Vec3D,
    w: Vec3D,
}

impl Camera {
    pub fn new(
        look_from: Vec3D,
        lookat: Vec3D,
        vup: Vec3D,
        pov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = pov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - lookat).unit_vector();
        let u = (Vec3D::cross(&vup, &w)).unit_vector();
        let v = Vec3D::cross(&w, &u);
        // println!("{:?} {:?}", u, v);
        Self {
            // lower_left: Vec3D::new(-half_width, -half_height, -1.0),
            lower_left: look_from
                - half_width * u * focus_dist
                - half_height * v * focus_dist
                - w * focus_dist,
            origin: look_from,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            lens_radius,
            v,
            u,
            w,
        }
    }
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3D {
    // let mut p = Vec3D::default();
    let mut rand = rand::thread_rng();
    loop {
        let p =
            2.0 * Vec3D::new(rand.gen::<f32>(), rand.gen::<f32>(), 0.0) - Vec3D::new(1.0, 1.0, 0.0);
        if Vec3D::dot(&p, &p) >= 1.0 {
            return p;
        }
    }
    // return p;
}
