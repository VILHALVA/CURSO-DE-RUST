fn main() {
    // Criando um array de strings com nomes de pessoas
    let nomes: [&str; 5] = ["Alice", "Bob", "Charlie", "David", "Eve"];

    // Exibindo os nomes usando um loop for
    println!("Lista de Nomes:");

    for nome in &nomes {
        println!("{}", nome);
    }

    // Modificando um elemento do array
    let mut nomes_mutaveis: [&str; 4] = ["Carla", "Fernando", "Grace", "Hugo"];
    nomes_mutaveis[3] = "Daniel"; // Modificando o último nome

    println!("\nLista de Nomes Atualizada:");

    for nome in &nomes_mutaveis {
        println!("{}", nome);
    }
}
