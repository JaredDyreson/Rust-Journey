fn my_function(x: i32) -> i32 {
    println!("{}", x);
    return x;
}

fn main() {
    // any variable with a leading _ will be interpreted as something the programmer does not
    // intend on using right away
    const _MY_CONST_VAR: u32 = 100 * 100; // this is a constant variable and will not change

    // MY_CONST_VAR = 100; this results in a compiler error

    let _x = 6; // variables by default are set to be immutable
                // x = 8; // this should fail

    let mut _y = 0;
    _y = 1; // this should not fail because it is set to be mutable
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z is: {}", z);

    let mut spaces = "      ";
    // spaces = spaces.len(); // you cannot reassign a different type to a variable
    //
    let guess: u32 = "42".parse().expect("Not a number");

    let floating = 2.0;

    let tup = (1, 2, 3);
    let (a, b, c) = tup; // destructure a tuple like this
    let value = tup.1; // you can access elements using the `.` notation

    println!("a: {}, b: {}, c: {}", a, b, c);

    let arr: [i32; 2] = [1, 2];
    let [g, h] = arr; // destructure an array like this
    println!("{}, {}", g, h);
    println!("{}", my_function(100));

    if z < 100 {
        println!("Under 100");
    } else {
        println!("Over or at 100");
    }

    let mut number = if z < 100 { 0 } else { 69 };

    let resultant = loop {
        number += 1;

        if number == 10 {
            break number * 2;
        }
    };
    println!("{}", resultant);
}
