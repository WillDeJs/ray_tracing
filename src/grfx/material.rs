use crate::grfx::vector::Vec3D;
use crate::grfx::ray::Ray;
use rand::Rng;
use crate::grfx::shape::HitRecord;
use crate::grfx::color::Color;
use crate::grfx::color;

///
///  Material trait
/// This trait allows different materials to be created. 
/// 
/// Exteing this trait means that an object being drawn will define how exactly the material will be displayed.
///  Or how rays will interact with a surface.
/// 
/// to implement this trait define the scatter method:
///     pub fn scatter(&self, ray : &Ray, records : &HitRecord, attenuation : &Vec3D, scattered : &Ray) -> bool; 
///     ray->
///     records->
///     attenuation ->
///     scattered -> calculated ray.
/// 
pub trait Material {
    fn scatter(&self, ray : &Ray, records : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool; 
}
#[derive(Debug)]
pub struct Lambertian {
    albedo : Color,
}

/// 
/// Material implementation for lambertian reflectance.
/// which defines a matte surface. See wiki link:
/// https://en.wikipedia.org/wiki/Lambertian_reflectance
/// 
impl Lambertian {
    pub fn new(albedo :Color) -> Self {Self{albedo}}
}

impl Material for Lambertian {
    fn scatter(&self, _ray : &Ray, record : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        let target = record.point + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.point, target - record.point);
        *attenuation = self.albedo.clone();
        // println!{"Att:{:?}",self.albedo};
        true
    }
}

fn random_in_unit_sphere() -> Vec3D {
    let mut vector : Vec3D;
    let mut rand = rand::thread_rng();
    loop {
        vector = 2.0 * Vec3D::new(rand.gen::<f32>(),rand.gen::<f32>(),rand.gen::<f32>() ) - Vec3D::new(1.0, 1.0, 1.0);
        if vector.squared_length() < 1.0 {
            break;
        }
    }
    vector
    
}


///
///  Material reflection struct for metals
/// 
/// 
#[derive(Debug)]
pub struct Metal {
    albedo : Color,
    fuzz : Option<f32>
}
impl Metal {
    pub fn new(albedo :Color, fuzz: Option<f32>) -> Self { Self{albedo, fuzz}}
}
impl Material for Metal {
    fn scatter(&self, ray : &Ray, record : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        // DEfault fuzz index is 1.0
        let fuzz = match self.fuzz {
            Some(value) => value.min(1.0),
            _ => 1.0,
        };
        let reflected = reflect(&(ray.direction().unit_vector()), &record.normal); 
        //record.point + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.point, reflected + random_in_unit_sphere() * fuzz );
        *attenuation = self.albedo.clone();
        // println!{"Att:{:?}",self.albedo};
        Vec3D::dot(&scattered.direction(), &record.normal) > 0.0
    }
}

fn reflect(v : &Vec3D, n:&Vec3D) -> Vec3D {
    return *v - (2.0 * Vec3D::dot(v,n)) * n;
}

fn schlick(cosine: f32 , ref_idx : f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

fn refract( v  : &Vec3D, n : &Vec3D, ni_over_nt: f32, refracted: &mut Vec3D) -> bool {
    let uv = v.unit_vector();
    let dt = Vec3D::dot(&uv, &n);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt  * (1.0 - dt * dt));
    if discriminant > 0.0 {
        *refracted = (ni_over_nt * (uv - dt * n)) - (n * discriminant.sqrt());
        return true;
    }
    false
}
pub struct Dialectric {
    ref_idx : f32
}

impl Dialectric {
    pub fn new(ref_idx: f32)->Self {
        Self{ref_idx}
    }
}
impl Material for Dialectric {
    fn scatter(&self, ray : &Ray, record : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        let outward_normal;
        let ni_over_nt;
        let mut rand = rand::thread_rng();
        let reflected = reflect(&ray.direction(), &record.normal);
        let mut refracted = Vec3D::default();
        let reflect_prob : f32;
        let cosine : f32;
        *attenuation = color::WHITE;  // white default for transparent glossy reflection 
        if Vec3D::dot(&ray.direction(), &record.normal) > 0.0 {
            outward_normal = -record.normal.clone();
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3D::dot(&ray.direction(), &record.normal) / ray.direction().length();
        } else {
            outward_normal = record.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = - Vec3D::dot(&ray.direction(), &record.normal) / ray.direction().length();
        }

        if refract(&ray.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            *scattered = Ray::new(record.point, reflected);
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray::new(record.point, reflected);
            reflect_prob = 1.0;
        }
        if rand.gen::<f32>() < reflect_prob {
            *scattered = Ray::new(record.point, reflected);
        } else {
            *scattered = Ray::new(record.point, refracted);
        }
        return true;
    }
}