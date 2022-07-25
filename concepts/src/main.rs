fn main() {
    const ARR: [i32;5] = [1,2,3,4,5];
    let mut s1 = String::from("helblo");
    let new_str = String::from("hi there");
    let x: (i32, f64, i32, usize, String) = (test_loop(24), test_fun(32.0), test_in_array(ARR), first_word(&mut s1), no_dangle(new_str));

    let r3 = &mut s1;
    println!("{}", r3);

    let five_hundred = x.3;

    print!("The value of y is: {five_hundred}")
}

fn test_fun(int: f64) -> f64 {
    int
}

fn test_loop(int: i32) -> i32 {
    let mut counter = 0;
    loop {
        if counter == int {
            return counter * 100
        }

        counter += 1;
    }
}

fn no_dangle(str: String)  -> String {
    let s = String::from(str);

    return s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("This is the i:{}", i);
            return i;
        }
    }

    s.len()
}

fn test_in_array(arr: [i32;5]) -> i32 {
    let mut sum:i32 = 0;
    for number in arr {
        println!("{sum}");
        sum += number;
    }

    return sum
}

// fn mutate_length(s: &mut String) -> usize {
//     let r1 = &s;
//     let r2 = &s;

//     println!("{},{}", r1, r2);

//     // let r3 = &mut s;

    
//     // println!("{}", r3);

//     s.push_str(", world");
//     s.len()
// }
