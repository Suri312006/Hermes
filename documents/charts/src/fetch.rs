use charming::{
    Chart,
    component::{Axis, Feature, Grid, Legend, SaveAsImage, Title, Toolbox},
    element::{AxisType, Tooltip, Trigger},
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

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Stacked Line"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().data(vec![
            "Email",
            "Union Ads",
            "Video Ads",
            "Direct",
            "Search Engine",
            "Affiliate Marketing",
        ]))
        .grid(
            Grid::new()
                .top("10%")
                .left("10%")
                .right("4%")
                .bottom("10%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Line::new()
                .name("Email")
                .stack("Total")
                .data(vec![120, 132, 101, 134, 90, 230, 210]),
        )
        .series(
            Line::new()
                .name("Union Ads")
                .stack("Total")
                .data(vec![220, 182, 191, 234, 290, 330, 310]),
        )
        .series(
            Line::new()
                .name("Video Ads")
                .stack("Total")
                .data(vec![150, 232, 201, 154, 190, 330, 410]),
        )
        .series(
            Line::new()
                .name("Direct")
                .stack("Total")
                .data(vec![320, 332, 301, 334, 390, 330, 320]),
        )
        .series(
            Line::new()
                .name("Search Engine")
                .stack("Total")
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        )
        .series(
            Line::new()
                .name("Affiliate Marketing")
                .stack("Total")
                .data(vec![180, 232, 210, 290, 250, 400, 370]),
        )
}
