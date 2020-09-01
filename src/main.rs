fn main() {
    let x = 10;
    println!("x equals {}",x);

    // We can't change this variable because it is imutable, right?
    // well, yes. We can use shadowing; declaring the same variable
    // using `let` and - of course - the same variable name.

    let x = "Tiago";
    println!("x equals {}", x);

    // seems like cheating, right?
    // for what I understand using `let` again will create the variable again
    // thus allowing us to 'change' it. If you think about it, it's not change
    // it's recreating. Besides, imutable doesn't necessary means const.
}
