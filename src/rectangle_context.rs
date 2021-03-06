
use {
    AddBevel,
    AddColor,
    AddImage,
    AddRound,
    BevelRectangleContext,
    Borrowed,
    Field,
    Image,
    ImageRectangleContext,
    RectangleColorContext,
    RoundRectangleContext,
    Value,
};
use internal::{
    CanRectangle,
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A rectangle context.
pub struct RectangleContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
}

impl<'a> Clone for RectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RectangleContext<'static> {
        RectangleContext {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RectangleContext<'a>, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, RectangleContext<'a>, Matrix2d> 
for RectangleContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> RectangleContext<'a> {
        RectangleContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for RectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RectangleContext<'a>, Rectangle> for RectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RectangleContext<'a> {
        RectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
        }
    }
}

impl<'a> AddColor<'a, RectangleColorContext<'a>> for RectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
        }
    }
}

#[test]
fn test_rgba() {
    use {Context, AddRectangle};

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color.get();
    assert_eq!(color[0], 1.0);
}

impl<'a> AddRound<'a, RoundRectangleContext<'a>> for RectangleContext<'a> {
    #[inline(always)]
    fn round(&'a self, radius: Radius) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Value(radius),
        }
    }
}

impl<'a> AddBevel<'a, BevelRectangleContext<'a>> for RectangleContext<'a> {
    #[inline(always)]
    fn bevel(&'a self, radius: Radius) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Value(radius),
        }
    }
}

impl<'a> AddImage<'a, ImageRectangleContext<'a>> for RectangleContext<'a> {
    fn image(&'a self, image: Image) -> ImageRectangleContext<'a> {
        ImageRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Value(image),
        }
    }
}

