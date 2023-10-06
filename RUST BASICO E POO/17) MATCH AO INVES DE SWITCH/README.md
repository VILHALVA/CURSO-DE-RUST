# MATCH AO INVES DE SWITC
Rust não possui uma instrução "switch" como em algumas outras linguagens de programação, como C e C++. Em vez disso, você pode usar a correspondência de padrões (pattern matching) com `match` para alcançar resultados semelhantes. O `match` é uma construção poderosa em Rust que permite que você compare um valor com vários padrões e execute código com base no padrão correspondente.

Aqui está um exemplo de como você pode usar `match` para substituir uma instrução "switch" em Rust:

```rust
fn main() {
    let escolha = 2;

    match escolha {
        1 => println!("Você escolheu a opção 1."),
        2 => println!("Você escolheu a opção 2."),
        3 => println!("Você escolheu a opção 3."),
        _ => println!("Escolha inválida."), // Padrão para qualquer outra escolha
    }
}
```

Neste exemplo, `match` é usado para verificar o valor da variável `escolha` e executar código com base no valor. Cada ramo do `match` corresponde a um valor específico de `escolha`. Se o valor de `escolha` corresponder a um dos padrões, o código associado será executado. Se nenhum dos padrões corresponder, o bloco `_` (underscore) será executado, que é o padrão para qualquer outra escolha.

Você pode usar `match` com diferentes tipos de padrões, incluindo valores constantes, intervalos, enums e até mesmo estruturas e tipos personalizados. Ele oferece grande flexibilidade e é uma alternativa poderosa à instrução "switch" encontrada em outras linguagens.