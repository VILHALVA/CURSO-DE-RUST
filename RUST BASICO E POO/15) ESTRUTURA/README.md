# ESTRUTURA
Em Rust, você pode criar suas próprias estruturas de dados personalizadas usando a palavra-chave `struct`. As estruturas permitem que você agrupe vários campos ou membros em um único tipo de dado. Aqui está um exemplo de como criar uma estrutura:

```rust
// Definindo uma estrutura chamada "Pessoa"
struct Pessoa {
    nome: String,
    idade: u32,
    cidade: String,
}

fn main() {
    // Criando uma instância da estrutura "Pessoa"
    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
        cidade: String::from("São Paulo"),
    };

    // Acessando os campos da estrutura
    println!("Nome: {}", pessoa1.nome);
    println!("Idade: {}", pessoa1.idade);
    println!("Cidade: {}", pessoa1.cidade);
}
```

Neste exemplo, definimos uma estrutura chamada `Pessoa` com três campos: `nome`, `idade` e `cidade`. No `main()`, criamos uma instância da estrutura `Pessoa` chamada `pessoa1` e inicializamos os campos com valores específicos. Em seguida, usamos a notação `.` para acessar e imprimir os valores dos campos.

Você também pode criar estruturas mutáveis, permitindo que você altere os valores dos campos após a criação da instância. Para fazer isso, você deve declarar a instância como mutável usando a palavra-chave `mut`, como mostrado abaixo:

```rust
fn main() {
    // Criando uma instância mutável da estrutura "Pessoa"
    let mut pessoa2 = Pessoa {
        nome: String::from("Bob"),
        idade: 25,
        cidade: String::from("Rio de Janeiro"),
    };

    // Alterando o valor do campo "idade"
    pessoa2.idade = 26;

    println!("Nome: {}", pessoa2.nome);
    println!("Idade: {}", pessoa2.idade);
    println!("Cidade: {}", pessoa2.cidade);
}
```

Este exemplo demonstra como criar e utilizar estruturas em Rust. As estruturas são úteis para organizar dados relacionados em seu programa de maneira mais clara e modular.

# ESTRUTURA É POO?
Não exatamente. Embora as estruturas em Rust e a programação orientada a objetos (POO) compartilhem alguns conceitos semelhantes, como agrupar dados relacionados, elas não são a mesma coisa. As estruturas em Rust são uma maneira de organizar dados, enquanto a POO é um paradigma de programação que envolve a organização de dados e funcionalidades em classes e objetos.

Aqui estão algumas diferenças entre estruturas em Rust e POO:

1. **Paradigma de Programação:** A POO é um paradigma de programação que se concentra na modelagem de objetos que podem ter propriedades (campos) e métodos (funções). As classes são usadas para definir o comportamento dos objetos. Em Rust, não há classes e objetos no mesmo sentido da POO tradicional.

2. **Herança e Polimorfismo:** A POO permite conceitos como herança e polimorfismo, que não são suportados da mesma forma em Rust. Rust usa um sistema de traits (traços) para implementar comportamentos polimórficos.

3. **Visibilidade e Encapsulamento:** Em POO, você pode usar modificadores de acesso para controlar a visibilidade dos membros de uma classe, como públicos, protegidos e privados. Em Rust, a visibilidade é controlada usando palavras-chave como `pub` e `pub(crate)`.

4. **Propriedades e Métodos:** Na POO, você pode definir propriedades e métodos diretamente em uma classe. Em Rust, você define métodos em implementações separadas (usando a palavra-chave `impl`).

5. **Herança vs. Composição:** Enquanto a POO incentiva a herança de classes, Rust favorece a composição de estruturas e a implementação de funcionalidades reutilizáveis por meio de traits.

Portanto, embora você possa usar estruturas em Rust para organizar dados e funções relacionadas, a programação orientada a objetos é um paradigma mais amplo que inclui outros conceitos e abordagens, como classes, objetos, herança e polimorfismo. Rust foi projetada para oferecer flexibilidade e segurança de memória sem depender estritamente do paradigma de POO. Em vez disso, Rust incentiva a composição e a implementação de funcionalidades reutilizáveis por meio de traits e outros mecanismos.