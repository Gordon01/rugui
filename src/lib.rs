#![no_std]

pub mod framebuffer;
pub mod geometry;
mod geometry_tests;
pub mod widgets;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
