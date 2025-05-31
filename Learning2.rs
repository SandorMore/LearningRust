fn main(){
    let a = 5;
    let b = 10;
    let mut c = 0:

    let list = vec![1, 2, 3, 4, 5];
    for i in list {
        println!("The value is: {}", i);
    }

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