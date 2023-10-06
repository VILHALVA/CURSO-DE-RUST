fn main() {
    // Variáveis para armazenar valores
    let numero1 = 10;
    let numero2 = 4;
    let mut resultado: i32; // Variável mutável para armazenar resultados intermediários

    // Adição
    resultado = numero1 + numero2;
    println!("{} + {} = {}", numero1, numero2, resultado);

    // Subtração
    resultado = numero1 - numero2;
    println!("{} - {} = {}", numero1, numero2, resultado);

    // Multiplicação
    resultado = numero1 * numero2;
    println!("{} * {} = {}", numero1, numero2, resultado);

    // Divisão
    resultado = numero1 / numero2;
    println!("{} / {} = {}", numero1, numero2, resultado);

    // Módulo (resto da divisão)
    resultado = numero1 % numero2;
    println!("{} % {} = {}", numero1, numero2, resultado);

    // Operadores unários
    let numero_negativo = -5;
    resultado = -numero_negativo;
    println!("Negativo de {} = {}", numero_negativo, resultado);

    // Incremento e Decremento (não suportados em Rust)
    // let mut contador = 5;
    // contador++; // Isso resultará em um erro de compilação

    // Usando operadores de atribuição
    let mut valor = 7;
    valor += 3; // Valor é incrementado em 3 (equivalente a valor = valor + 3)
    println!("Valor após incremento: {}", valor);

    // Note que Rust não suporta operadores de incremento e decremento (++ e --)
}
