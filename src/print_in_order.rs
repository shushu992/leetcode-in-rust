use std::sync::{Condvar, Mutex};

/**
 * https://leetcode.com/problems/print-in-order/
 */
struct Foo {
    mutex: Mutex<u8>,
    condvar: Condvar,
}

impl Foo {
    fn new() -> Self {
        Foo {
            mutex: Mutex::new(0),
            condvar: Condvar::new(),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        let mut x = self.mutex.lock().unwrap();
        while *x != 0 {
            x = self.condvar.wait(x).unwrap();
        }

        *x = 1;
        self.condvar.notify_all();

        // Do not change this line
        print_first();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let mut x = self.mutex.lock().unwrap();
        while *x != 1 {
            x = self.condvar.wait(x).unwrap();
        }

        *x = 2;
        self.condvar.notify_all();

        // Do not change this line
        print_second();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let mut x = self.mutex.lock().unwrap();
        while *x != 2 {
            x = self.condvar.wait(x).unwrap();
        }

        self.condvar.notify_all();

        // Do not change this line
        print_third();
    }
}
