
use {Field, Value, Borrowed, Matrix2d, Line};
use {Transform2d};
use vecmath::{translate, rotate_radians, scale, shear, multiply};

/// A line context.
pub struct LineContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
}

impl<'a> Transform2d<'a> for LineContext<'a> {
    #[inline(always)]
    fn trans(&'a self, x: f64, y: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let trans = translate(x, y);
                Value(multiply(&trans, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let rot = rotate_radians(angle);
                Value(multiply(&rot, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn scale(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let scale = scale(sx, sy);
                Value(multiply(&scale, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(alwyas)]
    fn shear(&'a self, sx: f64, sy: f64) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: {
                let shear = shear(sx, sy);
                Value(multiply(&shear, self.transform.get()))
            },
            line: Borrowed(self.line.get()),
        }
    } 
}