use orbtk::prelude::*;
use orbtk_backend::OrbtkBackend;
use plotters::prelude::{
    ChartBuilder, Circle, Color, DiscreteRanged, EmptyElement, Histogram, IntoDrawingArea,
    IntoFont, IntoLinspace, IntoSegmentedCoord, LineSeries, PathElement, PointSeries, Polygon,
    ShapeStyle, Text, BLACK, BLUE, RED, WHITE,
};

// OrbTk 2D drawing
#[derive(Clone, Default, PartialEq, Pipeline)]
struct Graphic2DPipeline;

impl RenderPipeline for Graphic2DPipeline {
    fn draw(&self, render_target: &mut RenderTarget) {
        let example = 2;
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
            if example == 1 {
                let mut chart = ChartBuilder::on(&root)
                    .x_label_area_size(40)
                    .y_label_area_size(40)
                    .margin(5)
                    .caption("Histogram Test", ("Roboto-Regular", 50))
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
            } else if example == 2 {
                root.fill(&WHITE).unwrap();
                let root = root.margin(10, 10, 10, 10);
                // After this point, we should be able to draw construct a chart context
                let mut chart = ChartBuilder::on(&root)
                    // Set the caption of the chart
                    .caption("This is our first plot", ("Roboto-Regular", 40).into_font())
                    // Set the size of the label region
                    .x_label_area_size(20)
                    .y_label_area_size(40)
                    // Finally attach a coordinate on the drawing area and make a chart context
                    .build_cartesian_2d(0f32..10f32, 0f32..10f32)
                    .unwrap();

                // Then we can draw a mesh
                chart
                    .configure_mesh()
                    // We can customize the maximum number of labels allowed for each axis
                    .x_labels(5)
                    .y_labels(5)
                    // We can also change the format of the label text
                    .y_label_formatter(&|x| format!("{:.3}", x))
                    .draw()
                    .unwrap();

                // And we can draw something in the drawing area
                chart
                    .draw_series(LineSeries::new(
                        vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                        &RED,
                    ))
                    .unwrap();
                // Similarly, we can draw point series
                chart
                    .draw_series(PointSeries::of_element(
                        vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                        5,
                        &RED,
                        &|c, s, st| {
                            EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                                + Circle::new((0, 0), s, st.filled()) // At this point, the new pixel coordinate is established
                                + Text::new(format!("{:?}", c), (10, 0), ("Roboto-Regular", 10).into_font())
                        },
                    ))
                    .unwrap();
            } else if example == 3 {
                let mut chart = ChartBuilder::on(&root)
                    .caption("Koch's Snowflake", ("Roboto-Regular", 50))
                    .build_cartesian_2d(-2.0..2.0, -1.5..1.5)
                    .unwrap();

                let mut snowflake_vertices = {
                    let mut current: Vec<(f64, f64)> = vec![
                        (0.0, 1.0),
                        ((3.0f64).sqrt() / 2.0, -0.5),
                        (-(3.0f64).sqrt() / 2.0, -0.5),
                    ];
                    for _ in 0..6 {
                        current = snowflake_iter(&current[..]);
                    }
                    current
                };

                chart
                    .draw_series(std::iter::once(Polygon::new(
                        snowflake_vertices.clone(),
                        &RED.mix(0.2),
                    )))
                    .unwrap();
                snowflake_vertices.push(snowflake_vertices[0]);
                chart
                    .draw_series(std::iter::once(PathElement::new(snowflake_vertices, &RED)))
                    .unwrap();
            } else if example == 4 {
                root.fill(&WHITE).unwrap();

                let root = root.titled("Image Title", ("Roboto-Regular", 30)).unwrap();

                let (upper, lower) = root.split_vertically(512);

                let x_axis = (-3.4f32..3.4).step(0.1);

                let mut cc = ChartBuilder::on(&upper)
                    .margin(5)
                    .set_all_label_area_size(50)
                    .caption("Sine and Cosine", ("Roboto-Regular", 20))
                    .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)
                    .unwrap();

                cc.configure_mesh()
                    .x_labels(20)
                    .y_labels(10)
                    .disable_mesh()
                    .x_label_formatter(&|v| format!("{:.1}", v))
                    .y_label_formatter(&|v| format!("{:.1}", v))
                    .draw()
                    .unwrap();

                cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))
                    .unwrap()
                    .label("Sine")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

                cc.draw_series(LineSeries::new(
                    x_axis.values().map(|x| (x, x.cos())),
                    &BLUE,
                ))
                .unwrap()
                .label("Cosine")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

                cc.configure_series_labels()
                    .border_style(&BLACK)
                    .draw()
                    .unwrap();

                /*
                // It's possible to use a existing pointing element
                 cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
                    (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
                    5,
                    Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
                )).unwrap();*/

                // Otherwise you can use a function to construct your pointing element yourself
                cc.draw_series(PointSeries::of_element(
                    (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
                    5,
                    ShapeStyle::from(&RED).filled(),
                    &|coord, size, style| {
                        EmptyElement::at(coord)
                            + Circle::new((0, 0), size, style)
                            + Text::new(format!("{:?}", coord), (0, 15), ("Roboto-Regular", 15))
                    },
                ))
                .unwrap();

                let drawing_areas = lower.split_evenly((1, 2));

                for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
                    let mut cc = ChartBuilder::on(&drawing_area)
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .margin_right(20)
                        .caption(format!("y = x^{}", 1 + 2 * idx), ("Roboto-Regular", 20))
                        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
                        .unwrap();
                    cc.configure_mesh().x_labels(5).y_labels(3).draw().unwrap();

                    cc.draw_series(LineSeries::new(
                        (-1f32..1f32)
                            .step(0.01)
                            .values()
                            .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                        &BLUE,
                    ))
                    .unwrap();
                }
            }
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
                        //.size(640, 480)
                        //.margin(20)
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
                .size(1200.0, 800.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

fn snowflake_iter(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut ret = vec![];
    for i in 0..points.len() {
        let (start, end) = (points[i], points[(i + 1) % points.len()]);
        let t = ((end.0 - start.0) / 3.0, (end.1 - start.1) / 3.0);
        let s = (
            t.0 * 0.5 - t.1 * (0.75f64).sqrt(),
            t.1 * 0.5 + (0.75f64).sqrt() * t.0,
        );
        ret.push(start);
        ret.push((start.0 + t.0, start.1 + t.1));
        ret.push((start.0 + t.0 + s.0, start.1 + t.1 + s.1));
        ret.push((start.0 + t.0 * 2.0, start.1 + t.1 * 2.0));
    }
    ret
}
