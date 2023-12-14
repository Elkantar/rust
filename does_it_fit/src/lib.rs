mod areas_volumes;

#[derive(Clone, Copy, Debug)]
pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

#[derive(Clone, Copy, Debug)]
pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    Pyramid,
    Parallelepiped,
}

use crate::areas_volumes as av;

pub fn area_fit(x: usize, y: usize, objects: GeometricalShapes, times: usize, a: usize, b: usize) -> bool {
    match objects {
        GeometricalShapes::Square => {
            let area = av::square_area(a);
            let total_area = x * y;
            area * times <= total_area
        } 
        GeometricalShapes::Circle => {
            let area = av::circle_area(a);
            let total_area = x * y;
            area * times as f64 <= total_area as f64
        }
        GeometricalShapes::Rectangle => {
            let area = av::rectangle_area(a, b);
            let total_area = x * y;
            area * times <= total_area
        }
        GeometricalShapes::Triangle => {
            let area = av::triangle_area(a, b);
            let total_area = x * y;
            area * times as f64 <= total_area as f64
        }
    }
}

pub fn volume_fit(x: usize, y: usize, z: usize, objects: GeometricalVolumes, times: usize, a: usize, b: usize, c: usize) -> bool {
    match objects {
        GeometricalVolumes::Cube => {
            let volume = av::cube_volume(a);
            let total_volume = x * y * z;
            volume * times <= total_volume
        }
        GeometricalVolumes::Sphere => {
            let volume = av::sphere_volume(a);
            let total_volume = x * y * z;
            volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Cone => {
            let volume = av::cone_volume(a, b);
            let total_volume = x * y * z;
            volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Pyramid => {
            let volume = av::triangular_pyramid_volume(av::triangle_area(a, b), c);
            let total_volume = x * y * z;
            volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Parallelepiped => {
            let volume = av::parallelepiped_volume(a, b, c);
            let total_volume = x * y * z;
            volume * times <= total_volume
        }
    }
}
