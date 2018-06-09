// Slices

pub fn slice() {
    let s = String::from("hoge fuga piyo");
    let num = first_word_index(&s);

    println!("{}'s first word ends at {}", s, num);

    // String Slices
    let s1 = String::from("hello, rust!");

    // String can be accessed by its index
    let hello = &s1[0..5];
    let rust = &s1[7..12];

    println!("{}, {}", hello, rust);

    let s2 = String::from("hogehoge f ugaaaaaaa");
    let s2_first = first_word(&s2);
    println!("{}", s2_first);

    // use standard String(not String::from) as argument
    let s3 = "star wars";
    let s3_first = first_word_str(&s3);
    println!("{}", s3_first);

    // slice from int array
    let arr = [1, 2, 3, 4, 5, 6, 7];
    let arr_slice = &arr[1..4];

    for e in arr_slice {
        println!("{}", e);
    }

    str_slice();
}

// it could work well
// however, returned value `s.len()` has no longer a mean if `s` is dropped
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// str & String type's slices are both called `&str`
fn str_slice() {
    let s = "hogehoge";
    let slice = &s[0..4];

    println!("{}", slice);
}
