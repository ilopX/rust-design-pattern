use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let accumulator = RefCell::new(vec![]);
    let mut ob = ValueObserver::<i32>::new();

    let subscriber = ob.subscribe(|val| {
        accumulator.borrow_mut().push(*val);
    });

    ob.notify(1);
    ob.notify(2);
    ob.unsubscribe(subscriber);
    ob.notify(3);
    ob.notify(4);

    assert_eq!(accumulator.borrow().as_slice(), &[1, 2]);
}


struct ValueObserver<'a, T> {
    subscribers: Vec<Subscriber<'a, T>>,
}

impl<'a, T> ValueObserver<'a, T> {
    fn new() -> Self {
        Self {
            subscribers: vec![],
        }
    }

    fn subscribe(&mut self, call: impl FnMut(&T) + 'a) -> Subscriber<'a, T> {
        let subscriber = Subscriber::new(call);
        let return_subscriber = subscriber.clone();
        self.subscribers.push(subscriber);

        return_subscriber
    }

    fn unsubscribe(&mut self, subscriber: Subscriber<'a, T>) {
        self.subscribers.retain(|val| val != &subscriber);
    }

    fn notify(&mut self, new_value: T) {
        for subscriber in self.subscribers.iter() {
            subscriber.call(&new_value);
        }
    }
}


type SubscriberCall<'a, T> = Rc<RefCell<dyn FnMut(&T) + 'a>>;

struct Subscriber<'a, T> {
    call: SubscriberCall<'a, T>,
}

impl<'a, T> Subscriber<'a, T> {
    fn new(call: impl FnMut(&T) + 'a) -> Self {
        Self {
            call: Rc::new(RefCell::new(call)),
        }
    }

    #[inline]
    fn call(&self, val: &T) {
        self.call.borrow_mut()(val);
    }
}

impl<'a, T> Clone for Subscriber<'a, T> {
    fn clone(&self) -> Self {
        Subscriber {
            call: Rc::clone(&self.call)
        }
    }
}

impl<'a, T> PartialEq<Self> for Subscriber<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.call, &other.call)
    }
}
