use std::sync::{Condvar, Mutex};

/**
 * https://leetcode.com/problems/print-foobar-alternately/
*/
struct FooBar {
    n: usize,
    mutex: Mutex<u8>,
    condvar: Condvar,
}

impl FooBar {
    fn new(n: usize) -> Self {
        let mutex = Mutex::new(0);
        FooBar {
            n,
            mutex,
            condvar: Condvar::new(),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut x = self.mutex.lock().unwrap();
            while *x != 0 {
                x = self.condvar.wait(x).unwrap();
            }

            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();

            *x = 1;
            self.condvar.notify_all();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut x = self.mutex.lock().unwrap();
            while *x != 1 {
                x = self.condvar.wait(x).unwrap();
            }

            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();

            *x = 0;
            self.condvar.notify_all();
        }
    }
}

/// 这个题目的测试用例，是AI写的
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// 辅助：跑一次 FooBar，收集输出字符串
    fn run(n: usize) -> String {
        let foobar = Arc::new(FooBar::new(n));
        let out = Arc::new(Mutex::new(String::new()));

        let f = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.foo(|| out.lock().unwrap().push_str("foo"));
            })
        };
        let b = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.bar(|| out.lock().unwrap().push_str("bar"));
            })
        };

        f.join().unwrap();
        b.join().unwrap();

        Arc::try_unwrap(out).unwrap().into_inner().unwrap()
    }

    #[test]
    fn test_n_1() {
        assert_eq!(run(1), "foobar");
    }

    #[test]
    fn test_n_2() {
        assert_eq!(run(2), "foobarfoobar");
    }

    #[test]
    fn test_n_5() {
        assert_eq!(run(5), "foobar".repeat(5));
    }

    #[test]
    fn test_n_100() {
        assert_eq!(run(100), "foobar".repeat(100));
    }

    /// 反复跑，每次都是新线程，检测是否有偶发错误
    #[test]
    fn test_stress_many_runs() {
        for i in 0..200 {
            let s = run(20);
            assert_eq!(s, "foobar".repeat(20), "failed at iteration {}", i);
        }
    }

    fn run_bar_first(n: usize) -> String {
        let foobar = Arc::new(FooBar::new(n));
        let out = Arc::new(Mutex::new(String::new()));

        // 先 spawn bar
        let b = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.bar(|| out.lock().unwrap().push_str("bar"));
            })
        };
        // 睡一会，确保 bar 先拿到锁进入 recv
        thread::sleep(std::time::Duration::from_millis(50));

        let f = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.foo(|| out.lock().unwrap().push_str("foo"));
            })
        };

        f.join().unwrap();
        b.join().unwrap();

        Arc::try_unwrap(out).unwrap().into_inner().unwrap()
    }

    #[test]
    fn test_bar_starts_first() {
        assert_eq!(run_bar_first(3), "foobarfoobarfoobar");
    }

    /// 在每次打印后 sleep/yield，模拟打印耗时
    #[test]
    fn test_slow_print_foo() {
        let n = 5;
        let foobar = Arc::new(FooBar::new(n));
        let out = Arc::new(Mutex::new(String::new()));

        let f = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.foo(|| {
                    out.lock().unwrap().push_str("foo");
                    // 打印后主动让出，制造"foo 打印完但还没 send"的窗口
                    thread::yield_now();
                });
            })
        };
        let b = {
            let foobar = Arc::clone(&foobar);
            let out = Arc::clone(&out);
            thread::spawn(move || {
                foobar.bar(|| {
                    thread::yield_now();
                    out.lock().unwrap().push_str("bar");
                });
            })
        };

        f.join().unwrap();
        b.join().unwrap();
        let s = Arc::try_unwrap(out).unwrap().into_inner().unwrap();
        assert_eq!(s, "foobar".repeat(n));
    }

    #[test]
    fn test_n_zero() {
        assert_eq!(run(0), "");
    }
}
