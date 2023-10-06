fn main() {
    // Exemplo com for loop
    println!("Usando o for loop:");
    for i in 1..=5 {
        println!("Número: {}", i);
    }

    // Exemplo com while loop
    println!("\nUsando o while loop:");
    let mut contador = 1;
    while contador <= 5 {
        println!("Número: {}", contador);
        contador += 1;
    }

    // Exemplo com loop infinito (loop) e saída com break
    println!("\nUsando o loop loop:");
    let mut outro_contador = 1;
    loop {
        println!("Número: {}", outro_contador);
        outro_contador += 1;

        if outro_contador > 5 {
            break;
        }
    }
}
