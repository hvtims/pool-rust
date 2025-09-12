pub mod areas_volumes;
use areas_volumes as av;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rectar = av::rectangle_area(x, y);
    match kind {
        av::GeometricalShapes::Square => av::square_area(a) * times <= rectar,
        av::GeometricalShapes::Circle => (av::circle_area(a) as usize) * times <= rectar,
        av::GeometricalShapes::Rectangle => av::rectangle_area(a, b) * times <= rectar,
        av::GeometricalShapes::Triangle => (av::triangle_area(a, b) as usize) * times <= rectar,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let container_volume = av::parallelepiped_volume(x, y, z);
    match kind {
        av::GeometricalVolumes::Cube => av::cube_volume(a) * times <= container_volume,
        av::GeometricalVolumes::Sphere => (av::sphere_volume(a) as usize) * times <= container_volume,
        av::GeometricalVolumes::Cone => (av::cone_volume(a, b) as usize) * times <= container_volume,
        av::GeometricalVolumes::TriangularPyramid => {
            let base_area = av::triangle_area(a, b);
            (av::triangular_pyramid_volume(base_area, c) as usize) * times <= container_volume
        },
        av::GeometricalVolumes::Parallelepiped => av::parallelepiped_volume(a, b, c) * times <= container_volume,
    }
}