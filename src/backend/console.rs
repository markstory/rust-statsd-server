use super::backend::Backend;
use super::super::buckets::Buckets;

pub struct Console {
    last_flush_time: u64,
    last_flush_length: u64
}


impl Console {
    pub fn new() -> Console {
        Console {
            last_flush_time: 0,
            last_flush_length: 0,
        }
    }
}


impl Backend for Console {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        println!("dumping buckets");
    }
}
