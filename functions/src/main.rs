fn main() {
    println!("Hello, world!");

    otra_funcion();

    segunda_funcion(67,22);

    let y = five();
    println!("{}",y);

    let j = sumar_uno(7);
    println!("{}",j)
}

fn otra_funcion(){
    println!("Otra función")
}

fn segunda_funcion(x: u32,y: u32) {
    println!("El valor de {}x{} es: {}",x,y,x*y)
}

fn five() -> i32 {
    5
}

fn sumar_uno(x: i32) -> i32 {
    x+1
}