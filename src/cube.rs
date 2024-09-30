use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;
use crate::texture::Texture;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
    pub texture: Option<Texture>, // Añadir el campo para la textura
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material, texture: Option<Texture>) -> Self {
        Cube {
            center,
            size,
            material,
            texture,
        }
    }

    // Método para calcular las coordenadas de textura (u, v)
    fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let local_point = *point - self.center;
        let half_size = self.size / 2.0;
    
        // Determinar la cara del cubo
        let (u, v) = if local_point.x.abs() > half_size - 1e-4 {
            // Cara X
            let u = (local_point.z / self.size) + 0.5;
            let v = (local_point.y / self.size) + 0.5;
            (u, v)
        } else if local_point.y.abs() > half_size - 1e-4 {
            // Cara Y
            let u = (local_point.x / self.size) + 0.5;
            let v = (local_point.z / self.size) + 0.5;
            (u, v)
        } else {
            // Cara Z
            let u = (local_point.x / self.size) + 0.5;
            let v = (local_point.y / self.size) + 0.5;
            (u, v)
        };
    
        // Asegúrate de que u y v estén en el rango [0, 1]
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
    
    
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let half_size = self.size / 2.0;
        let min_bound = self.center - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center + Vec3::new(half_size, half_size, half_size);

        let mut tmin = (min_bound.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (max_bound.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min_bound.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (max_bound.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (min_bound.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (max_bound.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        let t = if tmin > 0.0 { tmin } else { tmax };
        if t < 0.0 {
            return Intersect::empty();
        }

        let point = ray_origin + ray_direction * t;
        let normal = (point - self.center).normalize();

        // Comprobar si el cubo tiene una textura y aplicarla
        if let Some(texture) = &self.texture {
            let (u, v) = self.get_uv(&point);
            let tex_color = texture.get_color(u, v);
            let color = Material::new(
                crate::color::Color::new(tex_color[0], tex_color[1], tex_color[2]),
                self.material.specular,
                self.material.albedo,
                self.material.refractive_index,
            );
            return Intersect::new(point, normal, t, color);
        }

        Intersect::new(point, normal, t, self.material)
    }
}