fn main() {
    let media = 8.5;

    // Verifica a categoria do aluno com base em sua média
    let categoria = if media >= 9.0 {
        "Excelente"
    } else if media >= 7.0 {
        "Bom"
    } else if media >= 5.0 {
        "Regular"
    } else {
        "Insuficiente"
    };

    println!("Média: {:.1}", media);
    println!("Categoria: {}", categoria);
}
