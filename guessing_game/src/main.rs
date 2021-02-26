use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    //println!("El número secreto es: {}", numero_secreto);
    
    let mut numero = String::new();

    println!("Introduce un número: ");

    io::stdin()
        .read_line(&mut numero)
        .expect("Ha fallado la lectura del numero");

    let numero: u32 = numero.trim().parse().expect("Please type a number!");

    println!("Tu número es: {}",numero);

    //pasa a integer i32
    // let my_i32= numero.parse().unwrap_or(0);

    //println!("{}", type_of(numero));
    //println!("{}", type_of(&numero_secreto.to_string()));

    match numero.cmp(&numero_secreto) {
        Ordering::Less => println!("Más grande!!!"),
        Ordering::Greater => println!("Más pequeño!!!"),
        Ordering::Equal => println!("Has ganado!!!!!!!!!!!!!!!!!!"),
    }
    
}
