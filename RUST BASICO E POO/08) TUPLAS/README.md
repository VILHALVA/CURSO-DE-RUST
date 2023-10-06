# TUPLAS
Em Rust, uma tupla é uma estrutura de dados que pode conter um número arbitrário de elementos de diferentes tipos. As tuplas são úteis quando você deseja agrupar valores de maneira temporária sem a necessidade de criar uma estrutura personalizada.

Aqui está uma sintaxe básica para criar e usar tuplas em Rust:

```rust
fn main() {
    // Criando uma tupla com três elementos de tipos diferentes
    let minha_tupla = (42, "Olá, Mundo!", 3.14);

    // Acessando elementos da tupla usando a notação de ponto (índice baseado em zero)
    let primeiro_elemento = minha_tupla.0;
    let segundo_elemento = minha_tupla.1;
    let terceiro_elemento = minha_tupla.2;

    println!("Primeiro elemento: {}", primeiro_elemento);
    println!("Segundo elemento: {}", segundo_elemento);
    println!("Terceiro elemento: {}", terceiro_elemento);

    // Você pode atribuir nomes aos elementos da tupla (destructuring)
    let (idade, mensagem, pi) = minha_tupla;
    println!("Idade: {}", idade);
    println!("Mensagem: {}", mensagem);
    println!("PI: {}", pi);
}
```

Neste exemplo, criamos uma tupla chamada `minha_tupla` que contém três elementos de tipos diferentes: um inteiro, uma string e um número de ponto flutuante. Usamos a notação de ponto para acessar os elementos da tupla por índice e também demonstramos a técnica de "destructuring" para atribuir nomes aos elementos da tupla e acessá-los por esses nomes.

Tuplas são úteis quando você precisa agrupar valores temporariamente e não deseja criar uma estrutura personalizada para eles. Elas podem conter qualquer combinação de tipos e são uma parte importante da flexibilidade e expressividade da linguagem Rust.