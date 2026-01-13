mod areas_volumes;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    match kind {
        areas_volumes::GeometricalShapes::Square =>
            times * areas_volumes::square_area(a) <= areas_volumes::rectangle_area(x, y),
        areas_volumes::GeometricalShapes::Circle =>
            times * (areas_volumes::circle_area(a) as usize) <= areas_volumes::rectangle_area(x, y),
        areas_volumes::GeometricalShapes::Rectangle =>
            times * areas_volumes::rectangle_area(a, b) <= areas_volumes::rectangle_area(x, y),
        areas_volumes::GeometricalShapes::Triangle =>
            times * (areas_volumes::triangle_area(a, b) as usize) <=
                areas_volumes::rectangle_area(x, y),
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    match kind {
        areas_volumes::GeometricalVolumes::Cube =>
            times * areas_volumes::cube_volume(a) <= areas_volumes::parallelepiped_volume(x, y, z),
        areas_volumes::GeometricalVolumes::Cone =>
            times * (areas_volumes::cone_volume(a, b) as usize) <=
                areas_volumes::parallelepiped_volume(x, y, z),
        areas_volumes::GeometricalVolumes::Parallelepiped =>
            times * areas_volumes::parallelepiped_volume(a, b, c) <=
                areas_volumes::parallelepiped_volume(x, y, z),
        areas_volumes::GeometricalVolumes::Sphere =>
            times * (areas_volumes::sphere_volume(a) as usize) <=
                areas_volumes::parallelepiped_volume(x, y, z),
        areas_volumes::GeometricalVolumes::TriangularPyramid =>
            times * (areas_volumes::triangular_pyramid_volume(a as f64, b) as usize) <=
                areas_volumes::parallelepiped_volume(x, y, z),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangles_do_not_fit() {
        assert!(!area_fit((2, 4), areas_volumes::GeometricalShapes::Rectangle, 100, (2, 1)));
    }

    #[test]
    fn triangles_fit() {
        assert!(area_fit((5, 5), areas_volumes::GeometricalShapes::Triangle, 3, (5, 3)));
    }

    #[test]
    fn spheres_fit() {
        assert!(volume_fit(
            (5, 5, 5),
            areas_volumes::GeometricalVolumes::Sphere,
            3,
            (2, 0, 0)
        ));
    }

    #[test]
    fn parallelepiped_fits() {
        assert!(volume_fit(
            (5, 7, 5),
            areas_volumes::GeometricalVolumes::Parallelepiped,
            1,
            (6, 7, 4)
        ));
    }
}
