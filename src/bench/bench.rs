extern crate test;

use std::mem::replace;
#[cfg(test)]
use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: uint = 20;

// recursive fibonacci
fn fibonacci(n: uint) -> uint {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// iterative fibonacci
struct Fibonacci {
    curr: uint,
    next: uint,
}

impl Iterator<uint> for Fibonacci {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the
    // iter method of Bencher
    b.iter(|| {
        range(0, BENCH_SIZE).map(fibonacci).collect::<Vec<uint>>()
    })
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| {
        fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<uint>>()
    })
}

#[cfg(not(test))]
fn main() {
	println!("This program must be build and run with --test");
}
