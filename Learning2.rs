fn main(){
    let a = 5;
    let b = 10;


    let draw = |x, y| {
        println!("Drawing {} and {}", x, y);
    };
    draw(a, b);
    println!("{}", a + b);
}