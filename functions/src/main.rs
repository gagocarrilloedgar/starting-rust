fn main() {
    println!("This is the third chapter, functions!");

    another_function(5);

    // Declartion contain function bodies
    let y = {
        let x = 3;
        x + 1
    };

    println!("y is:{}", y);

    // Functions with retun values
    let x1 = five();
    println!("The value of x1 is:{}", x1);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x:i32) ->i32{
    x+1
}
