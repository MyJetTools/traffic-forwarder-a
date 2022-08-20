use std::sync::atomic::AtomicUsize;

pub struct ValueAccumulator {
    value: AtomicUsize,
}

impl ValueAccumulator {
    pub fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }

    pub fn append(&self, value: usize) {
        self.value
            .fetch_add(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn get_one_second(&self) -> usize {
        self.value.swap(0, std::sync::atomic::Ordering::SeqCst)
    }
}
