#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use crate::persistencelandscape::PointOrd;
use float_ord::FloatOrd;
use plotters::prelude::*;

/// # Panics
///
/// Will panic on fail to generate chart
/// Will panic on fail to draw chart
/// Will panic on fail to add data to chart
///
/// # Errors
///
/// Will error and propogate up on same conditions as panics
pub fn landscape(
    landscape: Vec<Vec<PointOrd>>,
    height: u32,
    width: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the data
    let to_plot: Vec<Vec<(f32, f32)>> = landscape
        .into_iter()
        .map(|s| s.into_iter().map(|PointOrd { x, y }| (x.0, y.0)).collect())
        .collect();
    // Get bounds
    let (x_lower, x_upper, y_lower, y_upper) = to_plot
        .iter()
        .flatten()
        .copied()
        .flat_map(|(x, y)| [(FloatOrd(x), FloatOrd(y))])
        .fold((FloatOrd(0.0), FloatOrd(0.0), FloatOrd(0.0), FloatOrd(0.0)), 
              |bounds, (x, y)| 
              (bounds.0.min(x),
               bounds.1.max(x),
               bounds.2.min(y),
               bounds.3.max(y),
               ));
    let root = BitMapBackend::new("output.png", (width, height)).into_drawing_area();
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
        .build_cartesian_2d(x_lower.0..x_upper.0, y_lower.0..y_upper.0)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{x:.3}"))
        .draw()?;

    let colors = vec![&RED, &GREEN, &BLUE];
    for (i, data) in to_plot.into_iter().enumerate() {
        chart
            .draw_series(LineSeries::new(data, colors[i % colors.len()]))?;
    }
    root.present()?;
    Ok(())
}
