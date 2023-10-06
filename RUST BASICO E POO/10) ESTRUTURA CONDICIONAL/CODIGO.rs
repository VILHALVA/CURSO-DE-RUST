fn main() {
    let idade = 25;

    if idade < 0 {
        println!("Idade inválida. Por favor, insira uma idade positiva.");
    } else if idade < 12 {
        println!("Você é uma criança.");
    } else if idade < 18 {
        println!("Você é um adolescente.");
    } else if idade < 65 {
        println!("Você é um adulto.");
    } else {
        println!("Você é um idoso.");
    }
}
