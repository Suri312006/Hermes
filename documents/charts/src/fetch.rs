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

// msg_db_size: 2^14
//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [21.417 ms 21.422 ms 21.427 ms]
//                         change: [-8.0889% -8.0520% -8.0159%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 3 outliers among 100 measurements (3.00%)
//   1 (1.00%) low mild
//   2 (2.00%) high mild
// Fetch/K = 10            time:   [49.810 ms 49.836 ms 49.864 ms]
//                         change: [-26.698% -26.640% -26.584%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 4 outliers among 100 measurements (4.00%)
//   4 (4.00%) high mild
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 33.2s, or reduce sample count to 10.
// Fetch/K = 100           time:   [331.74 ms 331.81 ms 331.90 ms]
//                         change: [-35.137% -35.118% -35.098%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 3 outliers among 100 measurements (3.00%)
//   1 (1.00%) high mild
//   2 (2.00%) high severe
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 315.1s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [3.1489 s 3.1492 s 3.1495 s]
//                         change: [-36.249% -36.172% -36.071%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 4 outliers among 100 measurements (4.00%)
//   1 (1.00%) low severe
//   3 (3.00%) high mild

// Send/K = 1              time:   [9.0410 ms 9.0452 ms 9.0496 ms]
//                         change: [-16.373% -16.296% -16.224%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high mild

// User/Create User        time:   [8.0575 ms 8.0869 ms 8.1416 ms]
//                         change: [-0.3537% +0.0475% +0.7622%] (p = 0.88 > 0.05)
//                         No change in performance detected.
// Found 8 outliers among 100 measurements (8.00%)
//   3 (3.00%) low mild
//   4 (4.00%) high mild
//   1 (1.00%) high severe
//
// msg_db_size: 2^15
//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [21.545 ms 21.559 ms 21.569 ms]
//                         change: [+0.5678% +0.6393% +0.6952%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) low severe
// Benchmarking Fetch/K = 10: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.1s, or reduce sample count to 90.
// Fetch/K = 10            time:   [50.428 ms 50.441 ms 50.454 ms]
//                         change: [+1.1547% +1.2133% +1.2758%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 2 outliers among 100 measurements (2.00%)
//   2 (2.00%) high mild
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 33.9s, or reduce sample count to 10.
// Fetch/K = 100           time:   [338.56 ms 338.64 ms 338.72 ms]
//                         change: [+2.0238% +2.0592% +2.0922%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 4 outliers among 100 measurements (4.00%)
//   4 (4.00%) low mild
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 322.2s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [3.2173 s 3.2177 s 3.2181 s]
//                         change: [+2.1579% +2.1737% +2.1908%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 6 outliers among 100 measurements (6.00%)
//   3 (3.00%) low severe
//   2 (2.00%) low mild
//   1 (1.00%) high mild

// Send/K = 1              time:   [9.1228 ms 9.1274 ms 9.1318 ms]
//                         change: [+0.8406% +0.9080% +0.9837%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 3 outliers among 100 measurements (3.00%)
//   2 (2.00%) low mild
//   1 (1.00%) high mild

// User/Create User        time:   [8.0656 ms 8.0698 ms 8.0737 ms]
//                         change: [-0.9009% -0.2119% +0.1685%] (p = 0.68 > 0.05)
//                         No change in performance detected.
// Found 4 outliers among 100 measurements (4.00%)
//   2 (2.00%) low severe
//   1 (1.00%) low mild
//   1 (1.00%) high mild
//
// msg_db_size: 2^16
//
//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [21.687 ms 21.694 ms 21.700 ms]
//                         change: [+0.5649% +0.6256% +0.6947%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 3 outliers among 100 measurements (3.00%)
//   2 (2.00%) low mild
//   1 (1.00%) high mild
// Benchmarking Fetch/K = 10: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.2s, or reduce sample count to 90.
// Fetch/K = 10            time:   [51.786 ms 51.804 ms 51.822 ms]
//                         change: [+2.6610% +2.7019% +2.7525%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 35.2s, or reduce sample count to 10.
// Fetch/K = 100           time:   [351.92 ms 352.02 ms 352.11 ms]
//                         change: [+3.9099% +3.9507% +3.9882%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 25 outliers among 100 measurements (25.00%)
//   5 (5.00%) low severe
//   2 (2.00%) low mild
//   10 (10.00%) high mild
//   8 (8.00%) high severe
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 335.5s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [3.3438 s 3.3489 s 3.3525 s]
//                         change: [+3.9191% +4.0794% +4.1928%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 9 outliers among 100 measurements (9.00%)
//   6 (6.00%) low severe
//   3 (3.00%) low mild

// Send/K = 1              time:   [9.2699 ms 9.2769 ms 9.2842 ms]
//                         change: [+1.5439% +1.6385% +1.7290%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 4 outliers among 100 measurements (4.00%)
//   4 (4.00%) high mild

// User/Create User        time:   [8.0764 ms 8.0811 ms 8.0861 ms]
//                         change: [+0.0556% +0.1411% +0.2195%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 8 outliers among 100 measurements (8.00%)
//   1 (1.00%) low severe
//   2 (2.00%) low mild
//   1 (1.00%) high mild
//   4 (4.00%) high severe
//
//
//
// msg_db_size: 2^17
//
//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [21.930 ms 21.939 ms 21.948 ms]
//                         change: [+1.0854% +1.1308% +1.1795%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Benchmarking Fetch/K = 10: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, or reduce sample count to 90.
// Fetch/K = 10            time:   [54.174 ms 54.193 ms 54.212 ms]
//                         change: [+4.5633% +4.6120% +4.6651%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 3 outliers among 100 measurements (3.00%)
//   3 (3.00%) high mild
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.6s, or reduce sample count to 10.
// Fetch/K = 100           time:   [375.65 ms 375.73 ms 375.82 ms]
//                         change: [+6.6975% +6.7349% +6.7754%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high severe
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 358.2s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [3.5850 s 3.5853 s 3.5855 s]
//                         change: [+6.9412% +7.0570% +7.2235%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 3 outliers among 100 measurements (3.00%)
//   3 (3.00%) low mild

// Send/K = 1              time:   [9.4871 ms 9.4907 ms 9.4942 ms]
//                         change: [+2.2144% +2.3049% +2.3886%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 2 outliers among 100 measurements (2.00%)
//   2 (2.00%) low mild

// User/Create User        time:   [8.0717 ms 8.0758 ms 8.0798 ms]
//                         change: [-0.1424% -0.0660% +0.0180%] (p = 0.10 > 0.05)
//                         No change in performance detected.
// Found 6 outliers among 100 measurements (6.00%)
//   2 (2.00%) low severe
//   1 (1.00%) low mild
//   2 (2.00%) high mild
//   1 (1.00%) high severe
//
// msg_db_size: 2^18
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [22.389 ms 22.396 ms 22.404 ms]
//                         change: [+2.0334% +2.0850% +2.1406%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 6 outliers among 100 measurements (6.00%)
//   4 (4.00%) high mild
//   2 (2.00%) high severe
// Benchmarking Fetch/K = 10: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, or reduce sample count to 80.
// Fetch/K = 10            time:   [58.733 ms 58.758 ms 58.782 ms]
//                         change: [+8.3616% +8.4231% +8.4753%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high mild
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 42.0s, or reduce sample count to 10.
// Fetch/K = 100           time:   [420.02 ms 420.10 ms 420.17 ms]
//                         change: [+11.775% +11.808% +11.842%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 10 outliers among 100 measurements (10.00%)
//   4 (4.00%) low mild
//   5 (5.00%) high mild
//   1 (1.00%) high severe
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 403.1s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [4.0283 s 4.0287 s 4.0291 s]
//                         change: [+12.353% +12.367% +12.380%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high mild

// Send/K = 1              time:   [9.9209 ms 9.9261 ms 9.9313 ms]
//                         change: [+4.5207% +4.5873% +4.6593%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high mild

// User/Create User        time:   [8.0801 ms 8.0906 ms 8.1063 ms]
//                         change: [+0.0463% +0.1830% +0.3914%] (p = 0.02 < 0.05)
//                         Change within noise threshold.
// Found 14 outliers among 100 measurements (14.00%)
//   2 (2.00%) low severe
//   5 (5.00%) low mild
//   5 (5.00%) high mild
//   2 (2.00%) high severe
//
// msg_db_size 2^19
//
//      Running benches/general.rs (/home/ec2-user/Hermes/target/release/deps/general-9ad382a0b3217084)
// Gnuplot not found, using plotters backend
// Fetch/K = 1             time:   [23.286 ms 23.295 ms 23.305 ms]
//                         change: [+3.9564% +4.0139% +4.0664%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 7 outliers among 100 measurements (7.00%)
//   3 (3.00%) high mild
//   4 (4.00%) high severe
// Benchmarking Fetch/K = 10: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.8s, or reduce sample count to 70.
// Fetch/K = 10            time:   [67.890 ms 67.937 ms 67.983 ms]
//                         change: [+15.525% +15.622% +15.713%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Benchmarking Fetch/K = 100: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 50.9s, or reduce sample count to 10.
// Fetch/K = 100           time:   [511.00 ms 511.40 ms 511.64 ms]
//                         change: [+21.624% +21.734% +21.796%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 2 outliers among 100 measurements (2.00%)
//   1 (1.00%) low severe
//   1 (1.00%) high mild
// Benchmarking Fetch/K = 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 494.5s, or reduce sample count to 10.
// Fetch/K = 1000          time:   [4.9427 s 4.9430 s 4.9433 s]
//                         change: [+22.681% +22.695% +22.710%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) low mild

// Send/K = 1              time:   [10.759 ms 10.763 ms 10.767 ms]
//                         change: [+8.3601% +8.4306% +8.5067%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high mild

// User/Create User        time:   [8.0822 ms 8.0876 ms 8.0928 ms]
//                         change: [-0.2406% -0.0373% +0.1103%] (p = 0.73 > 0.05)
//                         No change in performance detected.
// Found 11 outliers among 100 measurements (11.00%)
//   1 (1.00%) low severe
//   6 (6.00%) low mild
//   3 (3.00%) high mild
//   1 (1.00%) high severe
