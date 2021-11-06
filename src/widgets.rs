use super::framebuffer::*;
use super::geometry::*;

impl Framebuffer {
    pub fn progress_bar(&mut self, cords: &Coordinates, progress: u32, color: &Color) -> bool {
        if progress > 100 {
            return false;
        }

        self.draw_rect(cords, color);
        let position = cords.x0 + ((cords.x1 - cords.x0) as f32 * (progress as f32 / 100.0)) as i32;
        for x in cords.x_iter().skip(1) {
            let color = if x <= position { Color::Black } else { Color::White };
            self.draw_vertical_line(x,cords.y0 + 2, cords.y1 - 3, &color);
        }

        true
    }

    pub fn scroller(&mut self, cords: &Coordinates, position: i32, width: i32, 
        orient: Orientation, color: &Color) -> bool {
        if orient == Orientation::Vertical {
            self.draw_filled_rect(cords, &Color::White);

            let mid_x = (cords.x1 - cords.x0) / 2;
            self.draw_vertical_line( mid_x, cords.y0, cords.y1, color);
            let position = cords.y0 + ((cords.y1 - width - cords.y0) as f32 * (position as f32 / 100.0)) as i32;

            self.draw_rect(&Coordinates{
                x0: cords.x0, 
                x1: cords.x1, 
                y0: position, 
                y1: position+width}, color);
        }
        
        true
    }

    pub fn table(&mut self, cords: &Coordinates, rows: i32, columns: i32, color: &Color) -> bool {
        
        for x in cords.x_iter().step_by((cords.dx() / columns) as usize) {
            self.draw_vertical_line(x, cords.y0, cords.y1, color);
        }

        for y in cords.y_iter().step_by((cords.dx() / rows) as usize) {
            self.draw_line(&Coordinates{
                x0: cords.x0, 
                x1: cords.x1, 
                y0: y, 
                y1: y}, color);
        }

        true
    }
}