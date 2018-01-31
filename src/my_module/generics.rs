// generic data types

pub fn generics() {
    let number_list = vec![10, 73, 4, 473, 37, 734, 756];
    let result = largest(&number_list);
    println!("{}", result);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list [0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
