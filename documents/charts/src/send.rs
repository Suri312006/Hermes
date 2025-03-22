use charming::{
    Chart,
    component::{Axis, Feature, Grid, Legend, LegendItem, SaveAsImage, Title, Toolbox},
    element::{AxisType, NameLocation, Tooltip, Trigger},
    series::Line,
};

// Chart for Send operation
pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Send"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().data(vec![LegendItem::from("k = 1")]))
        .grid(
            Grid::new()
                .left("10%")
                .right("10%")
                .bottom("15%")
                .top("15%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name("Message DB size")
                .name_location(NameLocation::Center)
                .name_gap(30)
                .boundary_gap(false)
                .data(vec![
                    "2^14", "2^15", "2^16", "2^17", "2^18", "2^19", "2^20", "2^21",
                ]),
        )
        .y_axis(Axis::new().type_(AxisType::Value).name("Time in ms"))
        .series(Line::new().name("k = 1").data(vec![
            9.045, 9.127, 9.277, 9.491, 9.926, 10.763, 12.602, 16.362,
        ]))
}
