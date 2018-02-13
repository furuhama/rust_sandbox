// Functional features of Rust language

use std::thread;
use std::time::Duration;

pub fn functional_features() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    // basic closure annotation
    //
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // type defined closure annotation

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// warning

// let example_closure = |x| x;
//
// let s = example_closure(String::from("hello"));
// let n = example_closure(5); <- this occurs an error, cause when `s` defined
// example_closure's type is set as String(so we can't set u32 value)
