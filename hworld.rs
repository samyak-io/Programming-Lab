// use std::io

fn max(x: i32, y: i32, z: i32) -> i32 {
    let mut max = x;

    if y > max {
        max = y;
    }

    if z > max {
        max = z;
    }

    max
}


fn collatz(mut n: i32) -> i32 {
    let mut steps = 0;

    while n > 1 {
        if (n % 2) == 0 {
            n = n/2;
        }
        else {
            n = 3*n + 1;
        }
        steps += 1;
    }
    
    steps
}

fn collatz_recursive(n: i32, steps: i32) -> (i32, i32) {
    if n <= 1{
        return (n, steps);
    }
    else {
        if n % 2 == 0{
            collatz_recursive(n/2, steps + 1)
        }
        else {
            collatz_recursive(3*n + 1, steps+1)
        }
    }
}

fn max_collatz(start: i32, end: i32) -> (i32,i32,i32) {
    let mut max_steps = 0;
    for i in start..=end {
        let (_num, steps_from_func) = collatz_recursive(i, 0);
        if steps_from_func > max_steps {
            max_steps = steps_from_func;
        }
    }

    (start, end, max_steps)
}

fn fizzbuzz(start: i32, end: i32, num: &string, ) {

}

fn main(){
    println!("hello world");
    let mut x = 23;
    let y: i32 = 32;
    let z: i32 = 42;

    println!("x initially is {}", x);
    x = 43;
    println!("x after reassignment is {}", x);

    if x > y {
        if x > z {
            println!("{}", x);
        }
        else if z > x {
                println!("{}", z);
            }
    }
    else {
        if y > z {
            println!("{}", y);
        }
        else if z > y {
            println!("{}", z);
        }
    }

    let maximum = max(x, y, z);
    println!("maximum of the three using max function {}", maximum);


    let mut counter = 0;
    
    while counter < 10 {
        println!("counter value: {}", counter);
        counter +=1;
    }
    let number = 35;
    let steps = collatz(number);
    println!("number = {} and # of steps to reach 1 = {}", number, steps);

    let (num, steps_from_recursivec) = collatz_recursive(number, 0);
    println!("num: {}, steps to reach 1: {}", num, steps_from_recursivec);


    // max steps problem.
    let (s, e, max_steps) = max_collatz(1, 25);
    println!("max steps from {} to {}: {}", s, e, max_steps);

    println!("\n");

    // fizz buzz, to write in a clean, modular, quickly editable way.

    let to_print_when_3 = "Fizz";
    let to_print_when_5 = "Buzz";

    for i in 1 ..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}{}", to_print_when_3, to_print_when_5);
        }
        else if i % 3 == 0 {
            println!("{}", to_print_when_3);
        }
        else if i % 5 == 0 {
            println!("{}", to_print_when_5);
        }
        else {
            println!("{}", i);
        }
    }


}

// Homework: try and find max and min in an array in as few steps as possible.
// is there some sort of convergence in collatz. check?