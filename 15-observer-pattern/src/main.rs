use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, new_state: &str);
}

struct Subject {
    state: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    fn new(initial_state: &str) -> Self {
        Subject {
            state: String::from(initial_state),
            observers: RefCell::new(Vec::new()),
        }
    }

    fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn set_state(&mut self, new_state: &str) {
        self.state = String::from(new_state);
        for observer in self.observers.borrow().iter() {
            observer.update(&self.state);
        }
    }
}

struct Logger {
    name: String,
}

impl Observer for Logger {
    fn update(&self, new_state: &str) {
        let name = &self.name;
        println!("[{name}] state = '{new_state}'");
    }
}

struct Notifier {
    email: String,
}

impl Observer for Notifier {
    fn update(&self, new_state: &str) {
        let email = &self.email;
        println!("[Notifier] email {email}: '{new_state}'");
    }
}

fn main() {
    let mut subject = Subject::new("Initial");
    let logger = Rc::new(Logger {
        name: String::from("FileLogger"),
    });
    let notifier = Rc::new(Notifier {
        email: String::from("admin@example.com"),
    });
    subject.attach(Rc::clone(&logger) as Rc<dyn Observer>);
    subject.attach(Rc::clone(&notifier) as Rc<dyn Observer>);
    subject.set_state("State A");
    subject.set_state("State B");
}
