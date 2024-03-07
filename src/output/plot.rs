use std::{
    error::Error,
    ops::Range,
    path::Path, fs,
};

use plotters::prelude::*;

use crate::{prof::Event, util::group_by};

use super::Outputter;

pub struct PlotOutput<'a> {
    dir: &'a Path,
}

impl<'a> PlotOutput<'a> {
    pub fn new(dir: &'a str) -> Self {
        if let Err(_) = fs::read_dir(dir) {
            fs::create_dir(dir).unwrap();
        }
        PlotOutput { dir: Path::new(dir) }
    }
}

impl<'a> Outputter for PlotOutput<'a> {
    fn output(&self, events: &[Event]) {
        let grouped = group_by(events.iter(), |v| v.description.clone());
        for (description, event_group) in grouped {
            let png_path = self.dir.join(format!("{}.png", description));
            let xy = event_group
                .iter()
                .map(|v| (v.timestamp_millis, v.value));
            let x_spec = Range {
                start: event_group.first().unwrap().timestamp_millis as f32,
                end: event_group.last().unwrap().timestamp_millis as f32,
            };
            let y_spec = 0f32..event_group.iter().map(|v| v.value).max().unwrap() as f32;
            draw(
                &description,
                &event_group[0].unit,
                xy,
                x_spec,
                y_spec,
                &png_path,
            )
            .unwrap();
        }
    }
}

fn draw<'a, P>(
    description: &str,
    unit: &str,
    xy: impl Iterator<Item = (u32, u64)>,
    x_spec: Range<f32>,
    y_spec: Range<f32>,
    png_path: &'a P,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path> + ?Sized,
{
    let root = BitMapBackend::new(png_path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .margin_right(15)
        .margin_left(50)
        .set_left_and_bottom_label_area_size(40)
        .caption(description, IntoTextStyle::into_text_style(32, &root))
        .build_cartesian_2d(x_spec, y_spec)?;
    chart
        .configure_mesh()
        .x_desc("ms")
        .y_desc(unit)
        .axis_desc_style(IntoTextStyle::into_text_style(20, &root))
        .draw()?;
    chart.draw_series(LineSeries::new(xy.map(|(x, y)| (x as f32, y as f32)), &RED))?;

    root.present()?;

    Ok(())
}
