fn main(){
    let a = 5;
    let b = 10;
    let c = 0:

    while c < 5{
        println!("c is {}", c);
        c += 1;
    }
    let draw = |x, y| {
        println!("Drawing {} and {}", x, y);
    };
    draw(a, b);
    println!("{}", a + b);
}