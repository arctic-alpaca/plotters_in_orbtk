use orbtk::prelude::*;
use orbtk_backend::OrbtkBackend;
use plotters::prelude::{
    ChartBuilder, Circle, Color, EmptyElement, Histogram, IntoDrawingArea, IntoFont,
    IntoSegmentedCoord, LineSeries, PointSeries, Text, RED, WHITE,
};

// OrbTk 2D drawing
#[derive(Clone, Default, PartialEq, Pipeline)]
struct Graphic2DPipeline;

impl RenderPipeline for Graphic2DPipeline {
    fn draw(&self, render_target: &mut RenderTarget) {
        let mut render_context =
            RenderContext2D::new(render_target.width(), render_target.height());

        {
            let root = OrbtkBackend::new(
                &mut render_context,
                render_target.width(),
                render_target.height(),
            )
            .unwrap()
            .into_drawing_area();

            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                //.caption("Histogram Test", ("Roboto-Regular", 50.0))
                .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)
                .unwrap();

            chart
                .configure_mesh()
                .disable_x_mesh()
                .bold_line_style(&WHITE.mix(0.3))
                .y_desc("Count")
                .x_desc("Bucket")
                .axis_desc_style(("Roboto-Regular", 15))
                .draw()
                .unwrap();

            let data = [
                0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
            ];

            chart
                .draw_series(
                    Histogram::vertical(&chart)
                        .style(RED.mix(0.5).filled())
                        .data(data.iter().map(|x: &u32| (*x, 1))),
                )
                .unwrap();
        }
        render_target.draw(render_context.data());
    }
}

#[derive(Default, AsAny)]
pub struct MainViewState {}

impl MainViewState {}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, _ctx: &mut Context) {}
}

widget!(
    MainView<MainViewState> {
         render_pipeline: DefaultRenderPipeline
    }
);

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::new()
                .rows(Rows::create().push("*"))
                .child(
                    Canvas::new()
                        .attach(Grid::row(0))
                        .render_pipeline(DefaultRenderPipeline(Box::new(
                            Graphic2DPipeline::default(),
                        )))
                        .margin(4.0)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - canvas example")
                .position((100.0, 100.0))
                .size(800.0, 800.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
