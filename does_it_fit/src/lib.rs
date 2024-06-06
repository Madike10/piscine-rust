pub use areas_volumes::{circle_area, rectangle_area, square_area, triangle_area, GeometricalShapes};
pub use areas_volumes::*;
mod areas_volumes;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let rec_area = x * y;
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
    };
    (object_area * times) <= rec_area
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_volume = x * y * z;
    let object_volume = match objects {
        areas_volumes::GeometricalVolumes::Cube =>cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere =>sphere_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Cone =>cone_volume(a, b) as f64,
        areas_volumes::GeometricalVolumes::Pyramid =>triangular_pyramid_volume(a as f64, b) as f64,
        areas_volumes::GeometricalVolumes::Parallelepiped =>parallelepiped_volume(a, b, c) as f64,
    };
    (object_volume  * times as f64)  <= box_volume as f64
}
