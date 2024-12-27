fn main() {
    //println!("Hello, world!");
    let x = 4;
    println!("The value of x is: {}", x);
    {
        let x = 2;
        println!("The value of x is: {}", x);
    }
    let x = x + 1 ;
    println!("The value of x is: {}", x);

}
