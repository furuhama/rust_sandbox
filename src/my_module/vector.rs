// vector

pub fn vector() {
    //
    // basic
    //

    // init vector
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // set value when vector is initialized
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // update vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("{:?}", v3);

    // update vector (part 2)
    let mut v4 = vec![0, 1, 2, 3];
    v4.push(4);
    v4.push(5);
    v4.push(6);
    println!("{:?}", v4);

    // get element from vector
    let v5 = vec![10, 13, 15, 28];

    // get by index
    let third: &i32 = &v5[2];
    println!("3rd: {}", third);

    // get by index(when index out of range)
    // this causes an error
    // let nineth: &i32 = &v5[8];
    // println!("9th: {}", nineth);

    // get by get method
    let third: Option<&i32> = v5.get(2);
    println!("3rd: {:?}", third); // return Some(15)

    // get by get method(when index out of range)
    let tenth: Option<&i32> = v5.get(9);
    println!("10th: {:?}", tenth); // return None

    //
    // iterator
    //

    // iterate
    let v6 = vec![10, 28, 37, 93];
    for i in &v6 {
        println!("{}", i);
    }

    // change elements by iteration
    let mut v7 = vec![1, 92, 374, 27362];
    for i in &mut v7 {
        *i += 128
    }
    println!("{:?}", v7);

    //
    // enum
    //

    #[derive(Debug)]
    enum SpreadsheetCells {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCells::Int(3),
        SpreadsheetCells::Text(String::from("hoge")),
        SpreadsheetCells::Float(10.23),
    ];
    println!("{:?}", row);
}
