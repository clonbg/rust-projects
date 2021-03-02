fn main() {

    // variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // haciendo sombra shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = 6;
    println!("The value of y is: {}", y);

    //haciendo shadowing se puede cambiar el tipo
    let spaces = "   ";
    println!("{}",spaces);
    let spaces = spaces.len();
    println!("{}",spaces);


    // tuplas
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{}-{}-{}",five_hundred,six_point_four,one);

    // arrays
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}-{}", months[1], a[4]);

    let a = [3; 5];
    print!("{},{},{},{},{}",a[0],a[1],a[2],a[3],a[4]);
    println!("")
}
