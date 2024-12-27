fn main() {
    //println!("Hello, world!");ù
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("There are {} seconds in a minute", SECONDS_IN_MINUTE);

    //
    let x = 4;
    println!("The value of x is: {}", x);
    {
        let x = x - 2;
        println!("The value of x is: {}", x);
    }
    let x = x + 1 ;
    println!("The value of x is: {}", x);



}
