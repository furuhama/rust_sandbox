// generic data types

use std::cmp::PartialOrd;

pub fn generics() {
    // largest function
    let number_list = vec![10, 73, 4, 473, 37, 734, 756];
    let result = largest(&number_list);
    println!("largest number: {}", result);

    let char_list = vec!['y', 'm', 'a', 'z', 'n'];
    let result = largest(&char_list);
    println!("largest char: {}", result);

    // Point Struct
    let integer = Point { x: 1, y: 10 };
    let float = Point { x: 0.5, y: 23.58 };
    let int_and_float = Point { x: 35, y: 0.83 };
    println!("{:?}, {:?}, {:?}", integer, float, int_and_float);
    println!("{}", int_and_float.value_x());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list [0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn value_x(&self) -> &T {
        &self.x
    }
}
