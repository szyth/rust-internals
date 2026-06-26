use std::thread;

struct TaskRegistry {
    tasks: Vec<(&'static str, Box<dyn Fn() + Send + 'static>)>,
}

impl TaskRegistry {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn register(&mut self, s: &'static str, f: impl Fn() + Send + 'static) {
        self.tasks.push((s, Box::new(f)));
    }

    fn run_all(self) -> Vec<std::thread::JoinHandle<()>> {
        self.tasks
            .into_iter()
            .map(|(s, f)| {
                thread::spawn(move || {
                    println!("{}", s);
                    f();
                })
            })
            .collect()
    }
}

fn main() {
    let mut registry = TaskRegistry::new();

    let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let c = counter.clone();

    registry.register("increment", move || {
        c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });

    registry.register("log", || println!("task run"));

    let handles = registry.run_all();

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "final count: {}",
        counter.load(std::sync::atomic::Ordering::SeqCst)
    );
}
