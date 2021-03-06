use std::env;
use gnuplot::{Figure, Color};

use timeseries::io::csv;


fn main() {
    let file_path = env::args().nth(1).unwrap();
    let ts = csv::load_file(&file_path);

    let mut fg = Figure::new();
    fg.axes2d().lines(&ts.index, &ts.data, &[Color("blue")]);
    fg.show();
}

