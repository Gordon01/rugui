use super::framebuffer::*;
use super::geometry::*;

impl Framebuffer {
    pub fn progress_bar(&mut self, cords: &Coordinates, progress: u32, color: &Color) -> bool {
        if progress > 100 {
            return false;
        }

        self.draw_rect(cords, color);
        let position = cords.start.0 + (cords.dx() as f32 * (progress as f32 / 100.0)) as i32;
        for x in cords.x_iter().skip(1) {
            let color = if x <= position { Color::Black } else { Color::White };
            self.draw_vertical_line(x,cords.start.1 + 1, cords.end.1 - 1, &color);
        }

        true
    }

    pub fn scroller(&mut self, cords: &Coordinates, position: i32, width: i32, 
        orient: Orientation, color: &Color) -> bool {
        if orient == Orientation::Vertical {
            self.draw_filled_rect(cords, &Color::White);

            let mid_x = cords.dx() / 2;
            self.draw_vertical_line(mid_x, cords.start.1, cords.end.1, color);
            let position = cords.start.1 + ((cords.end.1 - width - cords.start.1) as f32 * (position as f32 / 100.0)) as i32;

            self.draw_rect(&Coordinates::new(
                (cords.start.0, position), (cords.end.0, position+width)), 
                 color);
        }
        
        true
    }

    pub fn table(&mut self, cords: &Coordinates, rows: i32, columns: i32, color: &Color) -> bool {
        
        for x in cords.x_iter().step_by((cords.dx() / columns) as usize) {
            self.draw_vertical_line(x, cords.start.1, cords.end.1, color);
        }

        for y in cords.y_iter().step_by((cords.dx() / rows) as usize) {
            self.draw_line(&Coordinates::new(
                (cords.start.0, y), (cords.end.0, y)), 
                 color);
        }

        true
    }
}