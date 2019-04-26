
use simplelog::*;
use std::fs::File;

pub fn init() {
    let path = "ShiftManager.log";
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(path).unwrap()),
        ]
    ).unwrap();
}