use charming::{
    Chart,
    component::{Axis, Feature, Grid, Legend, LegendItem, SaveAsImage, Title, Toolbox},
    element::{AxisType, NameLocation, Tooltip, Trigger},
    series::Line,
};
// msg_db_size = 2^14
// k = 1 : 3.3058 ms
// k = 10 : 4.3576 ms
// k = 100 : 13.774 ms
// k = 1000 : 113.19 ms
//
// send
// k = 1 : 3.4311 ms
//
// user
// creation : 1.6232 ms
//
//
//
// msg_db_size = 2^15
// k = 1 : 3.4443 ms
// k = 10 : 4.8045 ms
// k = 100 : 18.146 ms
// k = 1000 : 156.89 ms
//
// send
// k = 1 : 3.4597 ms
//
// user
// creation : 1.6329 ms
//
//
//
// msg_db_size = 2^16
// k = 1 : 3.5589 ms
// k = 10 : 5.8209 ms
// k = 100 : 27.658 ms
// k = 1000 : 238.70 ms
//
// send
// k = 1 : 3.3683 ms
//
// user
// creation : 1.6218 ms
//
//
//
// msg_db_size = 2^17
// k = 1 : 3.6178 ms
// k = 10 : 7.1238 ms
// k = 100 : 43.315 ms
// k = 1000 : 410.60 ms
//
// send
// k = 1 : 3.8439 ms
//
// user
// creation : 1.6324 ms
//
//
// msg_db_size = 2^18
// k = 1 : 3.9646 ms
// k = 10 : 11.236 ms
// k = 100 : 76.380 ms
// k = 1000 : 685.32
//
// send
// k = 1 : 3.9955 ms
//
// user
// creation : 1.68 ms
//
//
// msg_db_size = 2^19
// k = 1 : 4.68 ms
// k = 10 : 16.968 ms
// k = 100 : 134.09 ms
// k = 1000 : 1.308 ms
//
// send
// k = 1 : 4.4964 ms
//
// user
// creation : 1.6715
//
//
// msg_db_size = 2^20
// fetch
// k = 1 : 5.7658 ms
// k = 10 : 28.566 ms
// k = 100 : 263.36 ms
// k = 1000 : 2.4938 s
//
// send
// k = 1 : 5.9114 ms
//
// user
// creation : 1.69 ms
//
//
// msg_db_size = 2^21
// k = 1 : 8.9086 ms
// k = 10 : 55.251 ms
// k = 100 : 551.94 ms
// k = 1000 : 5.6199 s
//
// send
// k = 1 : 9.056 ms
//
// user
// creation : 1.72 ms
//
//
const TEN: &str = "k = 10";

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Fetch"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().data(vec![
            LegendItem::from("k = 1"),
            LegendItem::from(TEN),
            LegendItem::from("k = 100"),
            LegendItem::from("k = 1000"),
            LegendItem::from("From Paper"), // "k = 10",
                                            // "k = 100",
                                            // "k = 1000",
        ]))
        .grid(
            Grid::new()
                .left("51%")
                .right("2%")
                .bottom("5%")
                .contain_label(true),
        )
        .grid(
            Grid::new()
                .left("2%")
                .right("51%")
                .bottom("5%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name("Message DB size")
                .name_location(NameLocation::Center)
                .grid_index(0)
                .name_gap(30)
                .boundary_gap(false)
                .data(vec![
                    "2^14", "2 ^ 15", "2 ^ 16", "2 ^ 17", "2 ^ 18", "2 ^ 19", "2 ^ 20", "2 ^ 21",
                ]),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name("Message DB size")
                .grid_index(1)
                .name_location(NameLocation::Center)
                .name_gap(30)
                .boundary_gap(false)
                .data(vec![
                    "2^14", "2 ^ 15", "2 ^ 16", "2 ^ 17", "2 ^ 18", "2 ^ 19", "2 ^ 20", "2 ^ 21",
                ]),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Time in ms")
                .grid_index(0),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Time in ms")
                .grid_index(1),
        )
        .series(
            Line::new()
                .name("k = 1")
                .data(vec![
                    3.3058, 3.4443, 3.5589, 3.6178, 3.9646, 4.68, 5.7658, 8.9086,
                ])
                .x_axis_index(1)
                .y_axis_index(1),
        )
        .series(
            Line::new()
                .name("k = 10")
                .data(vec![
                    4.3576, 4.8045, 5.8209, 7.1238, 11.236, 26.968, 28.566, 55.251,
                ])
                .x_axis_index(1)
                .y_axis_index(1),
        )
        .series(
            Line::new()
                .name("k = 100")
                .data(vec![
                    13.774, 18.146, 27.658, 43.315, 76.380, 134.09, 263.36, 551.94,
                ])
                .x_axis_index(0)
                .y_axis_index(0),
        )
        .series(
            Line::new()
                .name("k = 1000")
                .data(vec![
                    113.19, 156.89, 238.7, 410.6, 685.2, 1308.0, 2493.8, 5619.9,
                ])
                .x_axis_index(0)
                .y_axis_index(0),
        )
        .series(
            Line::new()
                .name("From Paper")
                .data(vec![0.0, 0.0, 0.0, 0.0, 0., 0., 1.1, 0.0])
                .x_axis_index(1)
                .y_axis_index(1),
        )
    // .series(
    //     Line::new()
    //         .name("Direct")
    //         .data(vec![320, 332, 301, 334, 390, 330, 320]),
    // )
    // .series(
    //     Line::new()
    //         .name("Search Engine")
    //         .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
    // )
    // .series(
    //     Line::new()
    //         .name("Affiliate Marketing")
    //         .data(vec![180, 232, 210, 290, 250, 400, 370]),
    // )
}
