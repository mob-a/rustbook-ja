fn main() {
    
    println!("Hello, world!");
    another_function(5,6_000);
    sample1();
    println!("function five {}", five());
    println!("function five_ret {}", five_ret());
}

fn another_function(x:i32,y:i32){
    let z = x+y;
    println!("x + y = {}",z);
}
fn sample1(){
    let x=5;
    let y={
        let x=3;
        x+1
    };
    println!("y={}",y);
}

fn five() ->i32{
    5
}
fn five_ret() ->i32{
    return 5;
}