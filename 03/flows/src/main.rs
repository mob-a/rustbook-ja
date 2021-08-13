fn main() {
    let number = 3;

    if number == 3 {
        println!("number was three");
    }

    let x = if number == 3 { 4 } else { 5 };
    println!("x eq {}", x);

    loop {
        println!("again!");
        break;
    }

    let mut rest = 4;
    while rest != 0 {
        println!("{} !", rest);
        rest -= 1;
    }
    println!("lift off!");

    let a = [10, 20, 30, 40, 50];
    let mut ind = 0;
    while ind < 5 {
        println!("{}", a[ind]);
        ind += 1;
    }

    for elem in a.iter() {
        println!("{}", elem);
    }

    for num in (1..5).rev() {
        println!("{}!", num);
    }
    println!("Lift Off!");
}
