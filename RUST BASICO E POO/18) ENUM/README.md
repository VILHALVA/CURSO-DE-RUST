# CRIANDO ENUMS SIMPLES
Um enum simples em Rust é uma enumeração que permite que você defina um tipo personalizado com um conjunto limitado de valores possíveis. Aqui está um exemplo de um enum simples que representa estados de uma lâmpada:

```rust
// Definindo um enum chamado "EstadoLampada"
enum EstadoLampada {
    Desligada,
    Ligada,
    Piscando,
}

fn main() {
    // Criando variáveis que representam diferentes estados de lâmpada
    let l1 = EstadoLampada::Desligada;
    let l2 = EstadoLampada::Ligada;
    let l3 = EstadoLampada::Piscando;

    // Usando match para determinar o comportamento com base no estado da lâmpada
    match l1 {
        EstadoLampada::Desligada => println!("A lâmpada está desligada."),
        EstadoLampada::Ligada => println!("A lâmpada está ligada."),
        EstadoLampada::Piscando => println!("A lâmpada está piscando."),
    }

    match l2 {
        EstadoLampada::Desligada => println!("A lâmpada está desligada."),
        EstadoLampada::Ligada => println!("A lâmpada está ligada."),
        EstadoLampada::Piscando => println!("A lâmpada está piscando."),
    }

    match l3 {
        EstadoLampada::Desligada => println!("A lâmpada está desligada."),
        EstadoLampada::Ligada => println!("A lâmpada está ligada."),
        EstadoLampada::Piscando => println!("A lâmpada está piscando."),
    }
}
```

Neste exemplo, definimos um enum chamado `EstadoLampada` com três variantes: `Desligada`, `Ligada` e `Piscando`. Cada variante representa um estado possível da lâmpada.

No `main()`, criamos três variáveis (`l1`, `l2` e `l3`) que representam diferentes estados da lâmpada. Usamos a correspondência de padrões (`match`) para determinar o comportamento com base no estado da lâmpada e exibir mensagens correspondentes.

Este é um exemplo simples de como criar e usar um enum em Rust para representar valores discretos e fazer correspondência de padrões com base nesses valores.