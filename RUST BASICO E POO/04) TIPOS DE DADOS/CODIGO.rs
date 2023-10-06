fn main() {
    // Tipos Integrais
    let idade: i32 = 25;
    let salario: f64 = 45000.50;
    let eh_maior_de_idade: bool = true;

    println!("Idade: {}", idade);
    println!("Salário: {:.2}", salario);
    println!("É maior de idade? {}", eh_maior_de_idade);

    // Tipos de Caracteres
    let letra: char = 'A';
    println!("Letra: {}", letra);

    // Tipos de Tuplas
    let pessoa: (String, i32, char) = ("Alice".to_string(), 30, 'F');
    println!("Nome: {}, Idade: {}, Gênero: {}", pessoa.0, pessoa.1, pessoa.2);

    // Tipos de Vetores
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Primeiro número: {}", numeros[0]);

    // Tipo String
    let mensagem: String = "Olá, Rust!".to_string();
    println!("{}", mensagem);

    // Tipos de Enumerações (Enums)
    enum Status {
        Ativo,
        Inativo,
        Pausado,
    }

    let status_atual = Status::Ativo;
    match status_atual {
        Status::Ativo => println!("O status está ativo."),
        Status::Inativo => println!("O status está inativo."),
        Status::Pausado => println!("O status está pausado."),
    }

    // Tipos de Referências
    let mut contador = 10;
    let referencia = &contador;
    let referencia_mutavel = &mut contador;

    println!("Contador: {}", contador);
    println!("Referência: {}", referencia);
    println!("Referência Mutável: {}", referencia_mutavel);
}
