fn main() {
    loop { // diga adeus a sua cpu 100% de uso de 1 nucleo
        println!("Hello, world!");
        break // ufa n comeu seu nucleo
    }
    sorce_while();
    sorce_while_not_good();
    sorce_for();
    sorce_for_rev();
}

fn sorce_while() {
    let mut numero = 3;

    while numero != 0 {
        println!("{numero}!");
        numero = numero - 1;
    }
    println!("LIFTOFF!!!");
}

fn sorce_while_not_good() {
    let a = [1,2,3,4,5];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor eh: {}!",a[indice]);
        indice = indice + 1;
    }
}

fn sorce_for() {
    let matriz = [10,20,30,40,50];
    
    for elemento in matriz.iter() {
        println!("O valor eh: {}", elemento);
    }
}

fn sorce_for_rev() {
    for numero in (1..5).rev() {
        println!("O valor eh: {}", numero);
    }
}