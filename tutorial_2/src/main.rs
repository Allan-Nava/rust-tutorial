fn main() {
    let tup : (i32, bool, char) = (1, true, 'a');
    let tup_2 : (i8, bool, char) = (1, true, 'a');

    println!("The value of tup is: {:?} tup.1: {}, tup.0: {}", tup, tup.1, tup.0);
    println!("The value of tup_2 is: {:?} tup_2: {}, tup.0: {}", tup_2, tup_2.1, tup_2.0);


    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
}
