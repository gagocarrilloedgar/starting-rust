fn main() {
    println!("This is the third chapter: Expressions, world!");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statement

    let condition = true;
    let number = if condition {5} else{7};
    println!("number is: {}",number);

    //--- Loops functions
    loops_functions();

    // ---- While Expressions
    while_functions();

}

fn loops_functions(){

    let mut counter = 0;
    let result = loop{

        counter +=1;
        
        if counter ==10{
            break counter*2;
        }
    };

    println!("The result is {}", result);
}
fn while_functions(){
    let mut number=3;

    while number !=0{
        println!("{}",number);
        number -=1;
    }

    println!("LIFTOFF!!!");


    // looping through elements
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter(){
        println!("a is:{}",element)
    }

    // Another kind of for
    //.rev() reverses the countdown without including the final number
    // whic means it goes from N-1 to 1
    for number in (1..4).rev(){
        println!("{}",number);
    }
}
