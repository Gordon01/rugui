#[cfg(test)]
mod geometry_tests {
    use crate::coordinates::bounding_box::*;
    use crate::framebuffer::Color;
    use crate::geometry::{Circle, Ellipse};

    #[test]
    fn test_iter() {
        let cords = BBox::new((5, 10), (15, 15));
        assert!(cords.iter_x().count() == 11);
        assert!(cords.iter_y().count() == 6);
    }

    #[test]
    fn test_circle_from_bbox() {
        let cords = BBox::new((0, 10), (10, 20));
        let circle_orig = Circle::new((5, 15), 5, Color::Black);
        let circle = Circle::from_bbox(cords, Color::Black);
        assert_eq!(circle_orig, circle);

        let cords = BBox::new((0, 10), (10, 16));
        let circle_orig = Circle::new((3, 13), 3, Color::Black);
        let circle = Circle::from_bbox(cords, Color::Black);
        assert_eq!(circle_orig, circle);
    }

    #[test]
    fn test_ellipse_max_thickness() {
        let ellipse = Ellipse::new(10, 20, (10, 10), Color::Black);

        assert_eq!(ellipse.max_thickness(), 10);
        
        let ellipse = Ellipse::new(123, 100, (10, 10), Color::Black);
        assert_eq!(ellipse.max_thickness(), 100);
    }
}
