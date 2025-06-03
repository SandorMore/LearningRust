fn main(){
    let number = 10;
    num >> = 2; // This is a right shift operation, not a bitwise AND
    println!("The number is: {}", number);
    let a = 5;
    let b = 10;
    let mut c = 0:

    let list = vec![1, 2, 3, 4, 5];
    for i in list {
        println!("The value is: {}", i);
    }
    let string = "Sigma sigma";
    let mut iter = string.chars();
    while let Some(ch) = iter.next() {
        println!("{}", ch);
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

    let mut d = 0;
    loop {
        if d == 5 {
            break;
        }
        println!("d is {}", d);
        d += 1;
    }
}