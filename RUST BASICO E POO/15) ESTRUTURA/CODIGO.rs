// Definindo uma estrutura chamada "Carro"
struct Carro {
    modelo: String,
    cor: String,
    valor: f64,
}

fn main() {
    // Criando uma instância da estrutura "Carro"
    let carro1 = Carro {
        modelo: String::from("Sedan"),
        cor: String::from("Prata"),
        valor: 35000.00,
    };

    // Acessando e exibindo os campos da estrutura
    println!("Modelo: {}", carro1.modelo);
    println!("Cor: {}", carro1.cor);
    println!("Valor: R$ {:.2}", carro1.valor);

    // Criando outra instância da estrutura "Carro"
    let carro2 = Carro {
        modelo: String::from("SUV"),
        cor: String::from("Preto"),
        valor: 45000.00,
    };

    // Acessando e exibindo os campos da segunda instância
    println!("\nModelo: {}", carro2.modelo);
    println!("Cor: {}", carro2.cor);
    println!("Valor: R$ {:.2}", carro2.valor);
}
