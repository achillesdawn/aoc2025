// pub mod three;
// pub mod four;
// pub mod five;
// pub mod six;
// pub mod seven;
// pub mod eight;
// pub mod nine;
// pub mod ten;
pub mod eleven;

// pub mod grid;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%H:%M:%S%.3f".to_owned(),
        ))
        .init();
}
