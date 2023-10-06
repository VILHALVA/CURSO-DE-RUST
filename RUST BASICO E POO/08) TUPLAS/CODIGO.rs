fn main() {
    // Criando uma lista de tuplas com informações de alunos
    let alunos = [
        ("Alice", 18, 9.5),
        ("Bob", 17, 8.8),
        ("Charlie", 19, 7.2),
    ];

    // Iterando sobre a lista de tuplas e exibindo informações de cada aluno
    for aluno in &alunos {
        let (nome, idade, media) = *aluno; // Destructuring da tupla
        println!("Nome: {}", nome);
        println!("Idade: {}", idade);
        println!("Média: {:.2}", media);
        println!("---");
    }
}
