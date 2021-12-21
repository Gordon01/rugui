#[cfg(test)]
mod geometry_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::coordinates::bounding_box::*;

    #[test]
    fn test_iter() {
        let cords = BBox::new((5, 10).into(), (15, 15).into());
        assert!(cords.iter_x().count() == 11);
        assert!(cords.iter_y().count() == 6);
    }
}
