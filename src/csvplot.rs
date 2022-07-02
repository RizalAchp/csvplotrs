use crate::data::*;

#[derive(Debug)]
pub struct FileIOError(pub String);

#[derive(Debug)]
pub struct PlotCsvError(pub String);

impl fmt::Display for FileIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileIOError: {}", self.0)
    }
}

impl fmt::Display for PlotCsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PlotCsvError: {}", self.0)
    }
}

impl Error for FileIOError {}
impl Error for PlotCsvError {}

pub fn gen_plot<S, P>(
    _name: S,
    _in: P,
    _out: S,
    size: (u32, u32),
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let root_area = BitMapBackend::new(_out.as_ref(), size).into_drawing_area();
    root_area.fill(&WHITE)?;
    root_area.titled(_name.as_ref(), ("sans-serif", 40))?;
    let DataCsv { name, data } = get_data(_in.as_ref())?;

    let (xmin, xmax) = get_minmax_vec(&data, 0);
    println!("xmin: {} | xmax: {}", xmin, xmax);
    let (y_allmin, y_allmax) = get_minmax_all(&data);
    let mut charts = ChartBuilder::on(&root_area)
        .margin(i32::from(5))
        .set_all_label_area_size(i32::from(60))
        .build_cartesian_2d(xmin..xmax, y_allmin..y_allmax)?;

    charts
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    for idx in 1..5 {
        charts
            .draw_series(LineSeries::new(
                data.iter().map(|f| (f[0], f[idx])),
                &COLORDATA[idx.clone()],
            ))?
            .label(&name[idx])
            .legend(|d| PathElement::new(vec![d, (d.0 + 20, d.1)], &BLACK));
    }

    charts
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn gen_split_plot<S, P>(
    _name: S,
    _in: P,
    _out: S,
    size: (u32, u32),
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let root_area = BitMapBackend::new(_out.as_ref(), size).into_drawing_area();
    root_area.fill(&WHITE)?;
    root_area.titled(_name.as_ref(), ("sans-serif", 40))?;
    let DataCsv { name, data } = get_data(_in.as_ref())?;
    let size = data[0].len();
    if size < 3 {
        return Err(Box::new(PlotCsvError(format!(
            "Error: {}, {}",
            "Row Data csv is less than 3 rows. ",
            " you cannot genereate split plot with less than 3 rows data"
        ))));
    }

    let (upper, lower) = root_area.split_vertically(i32::from(256));

    let (x_min, x_max) = get_minmax_vec(&data, 0);
    let (y_min, y_max) = get_minmax_all(&data);

    let mut charts = ChartBuilder::on(&upper)
        .margin(i32::from(5))
        .set_all_label_area_size(i32::from(50))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    charts
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    charts
        .draw_series(LineSeries::new(data.iter().map(|x| (x[0], x[1])), &RED))?
        .label(&name[1])
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    charts
        .draw_series(LineSeries::new(data.iter().map(|x| (x[0], x[2])), &BLUE))?
        .label(&name[1])
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    charts
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    charts.draw_series(PointSeries::of_element(
        data.iter().map(|x| (x[0], x[2])),
        3,
        ShapeStyle::from(&RED).filled(),
        &|coord, size: i32, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(format!("{:?}", coord), (0, 12), ("sans-serif", 12))
        },
    ))?;

    let drawing_areas = lower.split_evenly((1, size - 2));

    for (drawing_area, idx) in drawing_areas.iter().zip(size - 3..) {
        let mut chart = ChartBuilder::on(&drawing_area)
            .x_label_area_size(i32::from(30))
            .y_label_area_size(i32::from(30))
            .margin_right(i32::from(20))
            .caption(&name[idx], ("sans-serif", i32::from(16)))
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
        chart.configure_mesh().x_labels(5).y_labels(3).draw()?;
        chart.draw_series(LineSeries::new(data.iter().map(|x| (x[0], x[idx])), &BLUE))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to out.png");
    Ok(())
}
