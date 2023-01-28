mod example;
mod triangle;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .try_init()
        .unwrap();
    example::run();
}
