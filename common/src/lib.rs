use std::time::SystemTime;

pub fn start_timer() -> SystemTime {
    SystemTime::now()
}

pub fn elapsed(since: &SystemTime) -> String {
    format!("{:?}", since.elapsed().expect("Time went backwards"))
}
