use std::io;

fn main() {
    // Cria um vetor vazio para armazenar os números
    let mut numeros: Vec<i32> = Vec::new();

    // Solicita ao usuário que insira números até que ele deseje parar
    loop {
        let mut entrada = String::new();

        println!("Digite um número (ou 'q' para parar):");
        
        io::stdin().read_line(&mut entrada)
            .expect("Falha ao ler a entrada");

        // Verifica se o usuário deseja parar
        if entrada.trim() == "q" {
            break;
        }

        // Converte a entrada para um número inteiro e adiciona ao vetor
        let numero: i32 = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida, por favor, insira um número válido.");
                continue;
            },
        };

        numeros.push(numero);
    }

    // Calcula a média dos números
    let soma: i32 = numeros.iter().sum();
    let media: f64 = soma as f64 / numeros.len() as f64;

    println!("Média dos números: {:.2}", media);

    // Exibe os números maiores que a média
    println!("Números maiores que a média:");
    for &numero in &numeros {
        if numero as f64 > media {
            println!("{}", numero);
        }
    }
}
