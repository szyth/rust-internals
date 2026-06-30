use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Config {
    name: String,
}

struct Subscriber {
    id: u32,
    config: Rc<Config>,
}

struct Watcher {
    subscribers: RefCell<Vec<Weak<Subscriber>>>,
}

impl Watcher {
    fn new() -> Self {
        Self {
            subscribers: RefCell::new(vec![]),
        }
    }
    fn register(&self, sub: &Rc<Subscriber>) {
        let weak = Rc::downgrade(sub);
        self.subscribers.borrow_mut().push(weak);
    }

    fn notify(&self, event: &str) {
        self.subscribers.borrow_mut().retain(|weak| {
            if let Some(sub) = weak.upgrade() {
                println!(
                    "Subscriber - id: {}, config: {}, event: {}",
                    sub.id, sub.config.name, event
                );
                true
            } else {
                false
            }
        });
    }
}

fn main() {
    let config = Rc::new(Config {
        name: "default".into(),
    });

    let watcher = Watcher::new();

    let sub1 = Rc::new(Subscriber {
        id: 1,
        config: Rc::clone(&config),
    });
    watcher.register(&sub1);
    {
        let sub2 = Rc::new(Subscriber {
            id: 2,
            config: Rc::clone(&config),
        });
        watcher.register(&sub2);
        watcher.notify("file_created"); // both sub1 and sub2 fires
    } // sub2 dropped here

    watcher.notify("file_deleted"); // only sub1 fires. sub2's dead Weak is pruned.
}
