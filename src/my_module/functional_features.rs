// Functional features of Rust language

use std::thread;
use std::time::Duration;

pub fn functional_features() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // use iterator
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("square_sum: {}", square_sum(10));
}

fn square_sum(x: isize) -> isize {
    (1..x).filter(|i| i % 2 == 0).map(|i| i * i).sum()
}

fn generate_workout(intensity: u32, random_number: u32) {
    // basic closure annotation
    //
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // argument type defined closure annotation
    //
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // struct using annotation

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// add _ as a prefix, it can disable unused warnings
pub struct _Counter {
    count: u32,
}

impl _Counter {
    pub fn _new() -> _Counter {
        _Counter { count: 0 }
    }
}

impl Iterator for _Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// warning

// let example_closure = |x| x;
//
// let s = example_closure(String::from("hello"));
// let n = example_closure(5); <- this occurs an error,
// because when `s` defined, example_closure's type is set as String(so we can't set u32 value)
