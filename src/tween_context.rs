use {Borrowed, Value, Field, Matrix2d};
use vecmath::{identity};
use {View};

/// An animation inbetweening context.
pub struct TweenContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
}

impl<'a> View<'a> for TweenContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> TweenContext<'a> {
        TweenContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

