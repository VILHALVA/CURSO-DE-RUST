fn main() {
    let inteiro_positivo: u32 = 42; // Número inteiro sem sinal (positivo)
    let inteiro_negativo: i64 = -10; // Número inteiro com sinal (negativo)

    println!("Inteiro positivo: {}", inteiro_positivo);
    println!("Inteiro negativo: {}", inteiro_negativo);

    // Operações matemáticas com números inteiros
    let soma = inteiro_positivo + inteiro_negativo;
    let subtracao = inteiro_positivo - inteiro_negativo;
    let multiplicacao = inteiro_positivo * inteiro_negativo;
    let divisao = inteiro_positivo / inteiro_negativo;

    println!("Soma: {}", soma);
    println!("Subtração: {}", subtracao);
    println!("Multiplicação: {}", multiplicacao);
    println!("Divisão: {}", divisao);
}
