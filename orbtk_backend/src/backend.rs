use orbtk::prelude::*;
use plotters_backend::text_anchor::{HPos, VPos};
use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontStyle, FontTransform,
};

#[derive(Debug)]
pub struct OrbtkError;

impl std::fmt::Display for OrbtkError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for OrbtkError {}

/// The drawing backend that is backed with a Cairo context
pub struct OrbtkBackend<'a> {
    render_ctx: RefCell<&'a mut RenderContext2D>,
    width: u32,
    height: u32,
}

impl<'a> OrbtkBackend<'a> {
    pub fn new(
        render_ctx: &'a mut RenderContext2D,
        width: f64,
        height: f64,
    ) -> Result<Self, OrbtkError> {
        let bytes = include_bytes!("Roboto-Regular.ttf");
        render_ctx.register_font("Roboto-Regular", bytes);
        let ret = Self {
            render_ctx: RefCell::new(render_ctx),
            width: width as u32,
            height: height as u32,
        };
        Ok(ret)
    }
    pub fn color_change(&self, color: &BackendColor) -> Color {
        /*Color::rgba(
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
            (color.alpha * 255.0) as u8,
        );*/
        Color::rgb(color.rgb.0, color.rgb.1, color.rgb.2)
    }
}

impl<'a> DrawingBackend for OrbtkBackend<'a> {
    type ErrorType = OrbtkError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_pixel");
        let mut render_ctx = self.render_ctx.borrow_mut();
        render_ctx.begin_path();
        render_ctx.set_alpha(color.alpha as f32);
        render_ctx.set_fill_style(utils::Brush::SolidColor(self.color_change(&color)));

        render_ctx.fill_rect(point.0 as f64, point.1 as f64, 1.0, 1.0);
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        println!("draw_line");
        render_ctx.begin_path();
        render_ctx.set_line_width(style.stroke_width() as f64);
        render_ctx.set_alpha(style.color().alpha as f32);
        render_ctx.set_stroke_style(utils::Brush::SolidColor(self.color_change(&style.color())));

        render_ctx.move_to(from.0 as f64, from.1 as f64);
        render_ctx.line_to(to.0 as f64, to.1 as f64);

        render_ctx.stroke();

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: (i32, i32),
        bottom_right: (i32, i32),
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        println!("draw_rect");
        render_ctx.begin_path();
        render_ctx.set_alpha(style.color().alpha as f32);
        let width = (bottom_right.0 - upper_left.0) as f64;
        let height = (bottom_right.1 - upper_left.1) as f64;
        if fill {
            render_ctx.set_fill_style(utils::Brush::SolidColor(self.color_change(&style.color())));
            render_ctx.fill_rect(upper_left.0 as f64, upper_left.1 as f64, width, height);
        } else {
            render_ctx.set_line_width(style.stroke_width() as f64);
            render_ctx
                .set_stroke_style(utils::Brush::SolidColor(self.color_change(&style.color())));
            render_ctx.stroke_rect(upper_left.0 as f64, upper_left.1 as f64, width, height);
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        println!("draw_path");
        render_ctx.begin_path();
        render_ctx.set_line_width(style.stroke_width() as f64);
        render_ctx.set_alpha(style.color().alpha as f32);
        render_ctx.set_stroke_style(utils::Brush::SolidColor(self.color_change(&style.color())));

        let iterator = path.into_iter();
        for (index, point) in iterator.enumerate() {
            if index == 0 {
                render_ctx.move_to(point.0 as f64, point.1 as f64);
            } else {
                render_ctx.line_to(point.0 as f64, point.1 as f64);
            }
        }

        render_ctx.stroke();

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: (i32, i32),
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        println!("draw_circle");
        render_ctx.begin_path();
        render_ctx.set_alpha(style.color().alpha as f32);

        if fill {
            render_ctx.set_fill_style(utils::Brush::SolidColor(self.color_change(&style.color())));
            render_ctx.arc(
                center.0 as f64,
                center.1 as f64,
                radius as f64,
                0.0,
                std::f64::consts::PI * 2.0,
            );
            render_ctx.fill();
        } else {
            render_ctx
                .set_stroke_style(utils::Brush::SolidColor(self.color_change(&style.color())));
            render_ctx.set_line_width(style.stroke_width() as f64);
            render_ctx.arc(
                center.0 as f64,
                center.1 as f64,
                radius as f64,
                0.0,
                std::f64::consts::PI * 2.0,
            );
            render_ctx.stroke();
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        println!("fill_polygon");
        render_ctx.begin_path();
        render_ctx.set_alpha(style.color().alpha as f32);
        render_ctx.set_fill_style(utils::Brush::SolidColor(self.color_change(&style.color())));
        for (index, point) in vert.into_iter().enumerate() {
            if index == 0 {
                render_ctx.move_to(point.0 as f64, point.1 as f64);
            } else {
                render_ctx.line_to(point.0 as f64, point.1 as f64);
            }
        }
        render_ctx.close_path();
        render_ctx.fill();

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: (i32, i32),
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        let (mut x, mut y) = (pos.0, pos.1);
        let degree = match style.transform() {
            FontTransform::None => 0.0_f64,
            FontTransform::Rotate90 => 90.0_f64,
            FontTransform::Rotate180 => 180.0_f64,
            FontTransform::Rotate270 => 270.0_f64,
        }
        .to_radians();

        if degree != 0.0 {
            println!("{}", degree);
            render_ctx.save();
            render_ctx.set_transform(
                degree.cos(),
                degree.sin(),
                -degree.sin(),
                degree.cos(),
                x as f64,
                y as f64,
            );
            x = 0;
            y = 0;
        }

        render_ctx.set_font_size(style.size());
        render_ctx.set_font_family("Roboto-Regular");

        let metrics = render_ctx.measure_text(text);
        println!("text: {}", text);
        match style.anchor().h_pos {
            HPos::Left => println!("h_pos: left"),
            HPos::Right => println!("h_pos: right"),
            HPos::Center => println!("h_pos: center"),
        };
        match style.anchor().v_pos {
            VPos::Top => println!("v_pos: top"),
            VPos::Center => println!("v_pos: center"),
            VPos::Bottom => println!("v_pos: bottom"),
        };

        let dx = match style.anchor().h_pos {
            HPos::Left => 0.0,
            HPos::Right => -metrics.width,
            HPos::Center => -metrics.width / 2.0,
        };
        let dy = match style.anchor().v_pos {
            VPos::Top => 0.0,
            VPos::Center => -metrics.height / 2.0,
            VPos::Bottom => -metrics.height,
        };

        render_ctx.begin_path();
        render_ctx.set_alpha(style.color().alpha as f32);
        render_ctx.set_fill_style(utils::Brush::SolidColor(self.color_change(&style.color())));

        //render_ctx.fill_text(text, f64::from(x) + dx, f64::from(y) + dy);
        if degree != 0.0 {
            render_ctx.fill_text(text, 0.0, 0.0);
        } else {
            render_ctx.fill_text(text, f64::from(x) + dx, f64::from(y) + dy);
        }
        /*render_ctx.fill_rect(
            f64::from(x) + dx,
            f64::from(y) + dy,
            metrics.width,
            metrics.height,
        );*/

        if degree != 0.0 {
            render_ctx.restore();
            render_ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        }

        Ok(())
    }

    fn estimate_text_size<TStyle: BackendTextStyle>(
        &self,
        text: &str,
        style: &TStyle,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let mut render_ctx = self.render_ctx.borrow_mut();
        let text_metrics = render_ctx.measure(text, style.size(), "Roboto-Regular");
        Ok((text_metrics.width as u32, text_metrics.height as u32))
    }
}
