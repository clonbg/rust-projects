use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // num aleatorio entre 1 y 100
    let numero_secreto = rand::thread_rng().gen_range(1..101);

    

    loop {
        // crea string vacío
        let mut numero = String::new();
        println!("Introduce un número: ");

        // entrada por teclado
        io::stdin()
            .read_line(&mut numero)
            .expect("Ha fallado la lectura del numero");

        // parsea a integer y si falla repite
        let numero: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu número es: {}",numero);

        // Compara y ejecuta
        match numero.cmp(&numero_secreto) {
            Ordering::Less => println!("Más grande!!!"),
            Ordering::Greater => println!("Más pequeño!!!"),
            Ordering::Equal => {
                println!("Has ganado!!!!!!!!!!!!!!!!!!");
                break;
            }
        }
    }

    
    
}
