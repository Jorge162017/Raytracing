use image::{DynamicImage, GenericImageView, Rgba};

pub struct Texture {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).expect("Failed to load texture");
        let (width, height) = image.dimensions();
        Texture { image, width, height }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Rgba<u8> {
        // Asegúrate de que u y v estén dentro del rango [0, 1]
        let u = u.fract();
        let v = v.fract();
    
        // Convertir coordenadas (u, v) en coordenadas de píxel
        let x = (u * (self.width as f32 - 1.0)).round() as u32;
        let y = ((1.0 - v) * (self.height as f32 - 1.0)).round() as u32; // Flip the v coordinate
    
        self.image.get_pixel(x, y)
    }
    
}
