use crate::persistencelandscape::PointOrd;
use plotters::prelude::*;

pub fn plot_landscape(landscape: Vec<Vec<PointOrd>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("5.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
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
    // chart
    //     .draw_series(LineSeries::new(
    //         vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
    //         &RED,
    //     ))
    //     .unwrap();
    let to_plot: Vec<Vec<(f32, f32)>> = landscape
        .into_iter()
        .map(|s| s.into_iter().map(|PointOrd { x, y }| (x.0, y.0)).collect())
        .collect();
    let colors = vec![&RED, &GREEN, &BLUE];
    for (i, data) in to_plot.into_iter().enumerate() {
        chart
            .draw_series(LineSeries::new(data, colors[i % colors.len()]))
            .unwrap();
    }
    root.present().unwrap();
    Ok(())
}
