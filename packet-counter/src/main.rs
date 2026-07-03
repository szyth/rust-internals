// CELL usage

use std::cell::Cell;

struct PacketCounter {
    packets: Cell<u64>,
    bytes: Cell<u64>,
    label: Cell<String>,
}

impl PacketCounter {
    fn new(label: &str) -> Self {
        Self {
            packets: Cell::new(0),
            bytes: Cell::new(0),
            label: Cell::new(label.to_string()),
        }
    }
    fn snapshot(&self) -> (u64, u64) {
        (self.packets.get(), self.bytes.get())
    }

    fn record(&self, bytes: u64) {
        self.packets.set(self.packets.get() + 1);
        self.bytes.set(self.bytes.get() + bytes);
    }
    fn change_label(&self, new_label: &str) {
        self.label.replace(new_label.to_string());
    }
}

fn main() {
    let c = PacketCounter::new("eth0");
    c.record(1024);
    c.record(64);

    c.change_label("wlan0");

    let (packets, bytes) = c.snapshot();
    println!("packets={}, bytes={}", packets, bytes);
    // label requires take/replace, skip it or access explicitly
}
