use super::ui_import::Stroke;

pub trait StrokeExt {
    fn modified_for_level(&self, level: usize) -> Stroke;
}

impl StrokeExt for Stroke {
    fn modified_for_level(&self, level: usize) -> Stroke {
        // Casting usize to i32 is not critical here. The value of `level` will always be a very
        // low number.  Even if a truncation or wrapping would occur, it would just mess up the
        // Visualization without any further harm.
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let factor = 0.5_f32.powi(level as i32);
        Stroke {
            width: self.width * factor,
            color: self.color.linear_multiply(factor),
        }
    }
}
