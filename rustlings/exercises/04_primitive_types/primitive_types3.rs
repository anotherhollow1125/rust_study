fn main() {
    let a = [true; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }

    println!("{}", a.into_iter().all(|b| b));
}
