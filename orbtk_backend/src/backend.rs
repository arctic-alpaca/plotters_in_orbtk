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
    render_ctx: &'a mut RenderContext2D,
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
            render_ctx,
            width: width as u32,
            height: height as u32,
        };
        Ok(ret)
    }
    pub fn color_change(&self, color: &BackendColor) -> Color {
        Color::rgba(
            (f32::from(color.rgb.0)) as u8,
            (f32::from(color.rgb.1)) as u8,
            (f32::from(color.rgb.2)) as u8,
            (color.alpha * 255.0) as u8,
        )
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
        self.render_ctx.begin_path();
        self.render_ctx
            .set_fill_style(utils::Brush::SolidColor(self.color_change(&color)));
        println!("{:#?}", point);
        self.render_ctx
            .fill_rect(point.0 as f64, point.1 as f64, 1.0, 1.0);
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        self.render_ctx.set_line_width(style.stroke_width() as f64);
        self.render_ctx
            .set_stroke_style(utils::Brush::SolidColor(self.color_change(&color)));
        self.render_ctx.begin_path();
        self.render_ctx.move_to(from.0 as f64, from.1 as f64);
        self.render_ctx.line_to(to.0 as f64, to.1 as f64);

        self.render_ctx.stroke();

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: (i32, i32),
        bottom_right: (i32, i32),
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.render_ctx.begin_path();
        let color = style.color();
        let width = (bottom_right.0 - upper_left.0) as f64;
        let height = (bottom_right.1 - upper_left.1) as f64;
        if fill {
            self.render_ctx
                .set_fill_style(utils::Brush::SolidColor(self.color_change(&color)));
            self.render_ctx
                .fill_rect(upper_left.0 as f64, upper_left.1 as f64, width, height);
        } else {
            self.render_ctx.set_line_width(style.stroke_width() as f64);
            self.render_ctx
                .set_stroke_style(utils::Brush::SolidColor(self.color_change(&color)));
            self.render_ctx
                .stroke_rect(upper_left.0 as f64, upper_left.1 as f64, width, height);
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        self.render_ctx.begin_path();
        self.render_ctx.set_line_width(style.stroke_width() as f64);
        self.render_ctx
            .set_stroke_style(utils::Brush::SolidColor(self.color_change(&color)));
        let iterator = path.into_iter();
        for (index, point) in iterator.enumerate() {
            if index == 0 {
                self.render_ctx.move_to(point.0 as f64, point.1 as f64);
            } else {
                self.render_ctx.line_to(point.0 as f64, point.1 as f64);
            }
        }
        self.render_ctx.stroke();

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: (i32, i32),
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        self.render_ctx.begin_path();
        if fill {
            self.render_ctx
                .set_fill_style(utils::Brush::SolidColor(self.color_change(&color)));
            self.render_ctx.arc(
                center.0 as f64,
                center.1 as f64,
                radius as f64,
                0.0,
                std::f64::consts::PI * 2.0,
            );
            self.render_ctx.fill();
        } else {
            self.render_ctx
                .set_stroke_style(utils::Brush::SolidColor(self.color_change(&color)));
            self.render_ctx.set_line_width(style.stroke_width() as f64);
            self.render_ctx.arc(
                center.0 as f64,
                center.1 as f64,
                radius as f64,
                0.0,
                std::f64::consts::PI * 2.0,
            );
            self.render_ctx.stroke();
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        self.render_ctx.begin_path();
        self.render_ctx
            .set_fill_style(utils::Brush::SolidColor(self.color_change(&color)));
        for (index, point) in vert.into_iter().enumerate() {
            if index == 0 {
                self.render_ctx.move_to(point.0 as f64, point.1 as f64);
            } else {
                self.render_ctx.line_to(point.0 as f64, point.1 as f64);
            }
        }
        self.render_ctx.close_path();
        self.render_ctx.fill();

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: (i32, i32),
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let (mut x, mut y) = (pos.0, pos.1);

        let degree = match style.transform() {
            FontTransform::None => 0.0,
            FontTransform::Rotate90 => 90.0,
            FontTransform::Rotate180 => 180.0,
            FontTransform::Rotate270 => 270.0,
        } / 180.0
            * std::f64::consts::PI;

        if degree != 0.0 {
            self.render_ctx.save();
            self.render_ctx.set_transform(
                degree.cos(),
                -degree.sin(),
                degree.sin(),
                degree.cos(),
                x as f64,
                y as f64,
            );
            x = 0;
            y = 0;
        }

        self.render_ctx.begin_path();
        self.render_ctx
            .set_fill_style(utils::Brush::SolidColor(self.color_change(&style.color())));

        self.render_ctx.set_font_size(style.size());
        self.render_ctx.set_font_family("Roboto-Regular");
        let metrics = self.render_ctx.measure_text(text);

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

        //
        self.render_ctx
            .fill_text(text, f64::from(x) + dx, f64::from(y) + dy);

        if degree == 0.0 {
            self.render_ctx.restore();
        }
        Ok(())
    }

    fn estimate_text_size<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let text_metrics = self
            .render_ctx
            .measure(text, style.size(), "Roboto-Regular");
        Ok((text_metrics.width as u32, text_metrics.height as u32))
    }
}
