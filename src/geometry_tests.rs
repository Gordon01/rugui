#[cfg(test)]
mod geometry_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::geometry::*;

    #[test]
    fn test_iter() {
        let cords = Coordinates::new((5, 15), (10, 15));
        assert!(cords.x_iter().count() == 11);
        assert!(cords.y_iter().count() == 6);
        assert!(cords.into_iter().count() == 66);
    }
}