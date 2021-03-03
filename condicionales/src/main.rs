use std::io;

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut cont = 0;
    loop {
        println!("again!");
        cont=cont+1;
        println!("cont: {}",cont);
        if cont == 9 {
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    //let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //Convertir entre Farenheits a Celsius
    println!("-----MENU-----");
    println!("1. Convertir de Fahrenheit a Celsius");
    println!("2. Convertir de Celsius a Fahrenheit");
    let mut menu = String::new();
    println!("Elija 1 o 2: ");
    io::stdin().read_line(&mut menu).expect("Ha fallado");
    // Sin trim no funciona la igualdad
    while menu.trim()!="1".to_string() &&  menu.trim()!="2".to_string() {
        println!("Elija 1 o 2: ");
        menu = "".to_string();
        io::stdin().read_line(&mut menu).expect("Ha fallado");
    }
    if menu.trim() == "1".to_string() {
        println!("de F a C");
        loop {
            println!("Introduzca los grados Fahrenheit: ");
            let mut numero = String::new();
            // entrada por teclado
            io::stdin()
            .read_line(&mut numero)
            .expect("Ha fallado la lectura del numero");

            // parsea a float y si falla repite
            let numero: f64 = match numero.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let resultado = (numero - 32.00)*(5.00/9.00);
            println!("{} grados Fahrenheit son {} grados Celsius",numero,resultado);
            break;
        }
    } else if menu.trim() == "2".to_string() {
        println!("de C a F");
        loop {
            println!("Introduzca los grados Celsius: ");
            let mut numero = String::new();
            // entrada por teclado
            io::stdin()
            .read_line(&mut numero)
            .expect("Ha fallado la lectura del numero");

            // parsea a float y si falla repite
            let numero: f64 = match numero.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let resultado = (numero * (9.00/5.00))+32.00;
            println!("{} grados Celsius son {} grados Fahrenheit",numero,resultado);
            break;
        } 
    }

    //Fibonacci
    let mut vector: Vec<u64> = Vec::new();
    vector.push(0);
    vector.push(1);
    let mut contador = 1;
    loop {
        let fibo = &vector[contador]+&vector[contador-1];
        vector.push(fibo);
        contador += 1;
        if contador == 20 {
            break;
        }
    }

    for i in &vector {
        println!("{}", i);
    }


    //The Twelve Days of Christmas
    let mut dias = 0;
    let orden = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    let cabecera1 = "\nOn the ".to_string();
    let cabecera2 = " day of Christmas,\nmy true love sent to me".to_string();
    let pie = "\nA partridge in a pear tree.\n".to_string();
    let pie2 = "\nAnd partridge in a pear tree.\n".to_string();
    let nuevo = ["","\nTwo turtle doves","\nThree French hens,","\nFour calling birds,","\nFive golden rings,","\nSix geese a-laying,","\nSeven swans a-swimming,","\nEight maids a-milking,","\nNine ladies dancing,","\nTen lords a-leaping,","\nEleven pipers piping,","\nTwelve drummers drumming,"];
    let mut texto = "".to_string();
    loop {
        texto.push_str(&cabecera1);
        texto.push_str(orden[dias]);
        texto.push_str(&cabecera2);
        for number in (0..dias+1).rev() {
            texto.push_str(nuevo[number])
        }
        if dias == 0 {
            texto.push_str(&pie);
        } else{
            texto.push_str(&pie2);
        } 
        dias += 1;
        if dias >= nuevo.len() {
            break;
        } 
    }
    println!("{}",texto);


}

