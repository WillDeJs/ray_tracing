use rand::Rng;
use ray_tracing::grfx::camera::Camera;
use ray_tracing::grfx::color::Color;
use ray_tracing::grfx::image::Image;
use ray_tracing::grfx::material::{Dialectric, Lambertian, Metal};
use ray_tracing::grfx::ray::Ray;
use ray_tracing::grfx::shape::{HitList, HitRecord, Hitable, Sphere};
use ray_tracing::grfx::vector::Vec3D;
use std::rc::Rc;

fn main() {
    let width = 600;
    let height = 400;
    let world = create_world();
    let pixels = render_world_pixels(world, width, height);
    let image = Image::from_colors(width, height, pixels);
    image.show();
}

/// Generate a bunch of spheres to show, at different centers and with different radi
fn create_world() -> HitList {
    // List of items in the image
    let mut world = HitList::new();
    let mut rand = rand::thread_rng();
    let background_sphere = Sphere::new(
        Vec3D::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Color::rgb(127, 127, 127))),
    );
    world.add(Rc::new(background_sphere));

    // generate random spheres at random places with different type of material
    for a in -12..12 {
        for b in -12..12 {
            let choose_mat = rand.gen::<f32>();
            let center = Vec3D::new(
                a as f32 + 0.9 * rand.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rand.gen::<f32>(),
            );
            if (center - Vec3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // difuse
                    let random_solid_shere = Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Color::rgb(
                            rand.gen::<u8>(),
                            rand.gen::<u8>(),
                            rand.gen::<u8>(),
                        ))),
                    );
                    world.add(Rc::new(random_solid_shere));
                } else if choose_mat < 0.8 {
                    // metal
                    let random_metalic_sphere = Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Color::rgb(rand.gen::<u8>(), rand.gen::<u8>(), rand.gen::<u8>()),
                            Option::Some(rand.gen::<f32>() * 0.5),
                        )),
                    );
                    world.add(Rc::new(random_metalic_sphere));
                } else {
                    let random_transparent_sphere =
                        Sphere::new(center, 0.2, Rc::new(Dialectric::new(2.0)));
                    world.add(Rc::new(random_transparent_sphere));
                }
            }
        }
    }

    // Some spheres to show
    let sphere_one = Sphere::new(
        Vec3D::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dialectric::new(2.0)),
    );
    let sphere_two = Sphere::new(
        Vec3D::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::rgb(102, 51, 26))),
    );
    let sphere_three = Sphere::new(
        Vec3D::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::rgb(179, 153, 127), Option::Some(0.0))),
    );

    // add all spheres to the world we'll visualize
    world.add(Rc::new(sphere_one));
    world.add(Rc::new(sphere_two));
    world.add(Rc::new(sphere_three));
    return world;
}

/// Color each one of the spheres according to their material designs
/// Return a list pixels (colors) after colorizing the spheres
///
fn render_world_pixels(world: HitList, width: u32, height: u32) -> Vec<Color> {
    let mut pixels = Vec::<Color>::new();
    let ns = 100; // for anti-aliasing (generating random pixels around for smoother view)

    // Camera used to calculate rays (lower_left, origin, width coordinate, height coordinatee)
    // let look_from = Vec3D::new(2.0, 0.0, 0.0);
    let look_from = Vec3D::new(6.0, 1.0, 2.0);
    // let look_at = Vec3D::new(-3.0, 0.0, 0.0);
    let look_at = Vec3D::new(4.0, 1.0, 1.0);
    let dist_to_focus = (look_from - look_at).length();
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3D::new(0.0, 1.0, 0.0),
        60.0,
        width as f32 / height as f32,
        0.01,
        dist_to_focus,
    );

    let mut rand = rand::thread_rng();
    // For all the pixels calculate the ray seen
    for j in (0..height).rev() {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        for i in 0..width {
            // I and J are the coordinates of a pixel in (width, hight)
            // Therefoer if I and J are pixels then u and v are blocks/chunks/ray heads made by those pixels.
            let mut col: Color;
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand.gen::<f32>()) / width as f32;
                let v = (j as f32 + rand.gen::<f32>()) / height as f32;
                let r: Ray = cam.ray(u, v);

                col = color(&r, &world, 0);

                // Deal with teh colors separately since each item is u8 in size
                // otherwise they can overflow and crash
                red += col.r() as f32 / 255.0;
                green += col.g() as f32 / 255.0;
                blue += col.b() as f32 / 255.0;
            }

            // gama calculation of color
            red /= ns as f32;
            green /= ns as f32;
            blue /= ns as f32;

            red = red.sqrt();
            green = green.sqrt();
            blue = blue.sqrt();

            col = Color::rgb(
                (red * 255.99) as u8,
                (green * 255.99) as u8,
                (blue * 255.99) as u8,
            );

            // pixel container to be written to image
            pixels.push(col);
        }
    }
    pixels
}

/// Helper: Aids when coloring any hittalbe item (spheres in this case)
/// Uses their material and difueses/reflects according to the type of material.
///
fn color(r: &Ray, world: &impl Hitable, depth: i32) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, f32::MAX, &mut rec) {
        let mut scattered: Ray = Ray::new(Vec3D::new(0.0, 0.0, 0.0), Vec3D::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::rgb(0, 0, 0);
        if depth < 50
            && rec
                .material
                .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation.difuse(&color(&scattered, world, depth + 1));
        } else {
            return Color::rgb(0, 0, 0);
        }
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::rgb(255, 255, 255) * (1.0 - t) + Color::rgb(127, 180, 255) * t
}
