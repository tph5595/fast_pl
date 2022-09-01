use crate::persistencelandscape::PointOrd;
use float_ord::FloatOrd;
use plotters::prelude::*;

pub fn plot_landscape(landscape: Vec<Vec<PointOrd>>) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the data
    let to_plot: Vec<Vec<(f32, f32)>> = landscape
        .into_iter()
        .map(|s| s.into_iter().map(|PointOrd { x, y }| (x.0, y.0)).collect())
        .collect();
    // Get bounds
    let lower_bound_x = to_plot
        .to_vec()
        .into_iter()
        .flatten()
        .flat_map(|(x, ..)| [FloatOrd(x)])
        .min()
        .unwrap()
        .0;
    let upper_bound_x = to_plot
        .to_vec()
        .into_iter()
        .flatten()
        .flat_map(|(x, ..)| [FloatOrd(x)])
        .max()
        .unwrap()
        .0;
    let lower_bound_y = to_plot
        .to_vec()
        .into_iter()
        .flatten()
        .flat_map(|(.., y)| [FloatOrd(y)])
        .min()
        .unwrap()
        .0;
    let upper_bound_y = to_plot
        .to_vec()
        .into_iter()
        .flatten()
        .flat_map(|(.., y)| [FloatOrd(y)])
        .max()
        .unwrap()
        .0;
    let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    match root.fill(&WHITE) {
        Ok(_) => (),
        _ => {
            unreachable!("Could not set backgrond color")
        }
    };
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(lower_bound_x..upper_bound_x, lower_bound_y..upper_bound_y)
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

    let colors = vec![&RED, &GREEN, &BLUE];
    for (i, data) in to_plot.into_iter().enumerate() {
        chart
            .draw_series(LineSeries::new(data, colors[i % colors.len()]))
            .unwrap();
    }
    root.present().unwrap();
    Ok(())
}
