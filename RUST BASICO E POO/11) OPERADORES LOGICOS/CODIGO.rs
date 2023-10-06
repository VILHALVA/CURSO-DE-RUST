fn main() {
    let numero = 15;

    // Verifica se o número está no intervalo de 10 a 20
    let esta_no_intervalo = (numero >= 10) && (numero <= 20);

    if esta_no_intervalo {
        println!("O número {} está no intervalo de 10 a 20.", numero);
    } else {
        println!("O número {} não está no intervalo de 10 a 20.", numero);
    }

    // Verifica se o número está fora do intervalo de 30 a 40
    let esta_fora_do_intervalo = (numero < 30) || (numero > 40);

    if esta_fora_do_intervalo {
        println!("O número {} está fora do intervalo de 30 a 40.", numero);
    } else {
        println!("O número {} está dentro do intervalo de 30 a 40.", numero);
    }

    // Verifica se o número não é zero
    let nao_e_zero = numero != 0;

    if nao_e_zero {
        println!("O número {} não é zero.", numero);
    } else {
        println!("O número {} é zero.", numero);
    }
}
