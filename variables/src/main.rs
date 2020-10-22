fn main() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = x + 6;
    println!("The value of x is:{}", x);

    // addition
    let sum = 5 + 10;
    println!("The value of sum is:{}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is:{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is:{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is:{}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is:{}", remainder);

    //---- Specifying type anotation -----
    let f: bool = false;
    println!("The value of f is:{}", f);

    // Charts
    let z1 = 'z';
    let z2 = 'ℤ';
    let z = '😻';
    println!("The value of z is:{} z2 is:{} z2 is:{}", z, z2, z1);

    //------- The Tupple types
    //A tuple is a general way of grouping together a number of values
    //with a variety of types into one compound type. Tuples have a fixed length:
    //once declared, they cannot grow or shrink in size.difference

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The tup values are:{},{},{}", x, y, z);
    println!(
        "Another way to show tup values:{},{},{}",
        tup.0, tup.1, tup.2
    );

    //------- The Array types

    // let a [type; number of elemets]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a.0:{}", a[0]);

    //or
    let b = [3; 5]; //[value; size]
    println!("b.0 = {}", b[0]);

    another_function();
}

fn another_function() {
    println!("Another function.");
}
