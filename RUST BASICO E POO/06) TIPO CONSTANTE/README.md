# TIPO CONSTANTE:

Em Rust, você pode criar constantes usando a palavra-chave `const`. As constantes são valores imutáveis que têm um tipo de dados associado a eles e são definidas em tempo de compilação. Aqui está como você pode declarar e usar constantes em Rust:

```rust
const PI: f64 = 3.14159265359;
const IDADE_MINIMA: u32 = 18;
const NOME: &str = "Alice";

fn main() {
    println!("Valor de PI: {}", PI);
    println!("Idade mínima: {}", IDADE_MINIMA);
    println!("Nome: {}", NOME);
}
```

Neste exemplo, definimos três constantes:

1. `PI` é uma constante de ponto flutuante (`f64`) que representa o valor de π (pi).
2. `IDADE_MINIMA` é uma constante sem sinal (`u32`) que representa a idade mínima.
3. `NOME` é uma constante que armazena uma referência a uma string (`&str`) com um nome.

As constantes devem sempre ter um tipo de dados associado a elas e devem ser inicializadas com um valor constante em tempo de compilação. Note que, ao contrário das variáveis, as constantes não podem ser mutáveis e seu valor não pode ser alterado após a definição.

É uma prática comum usar letras maiúsculas e sublinhados para nomear constantes em Rust, para distingui-las de variáveis. As constantes são uma escolha apropriada quando você tem um valor que não deve ser alterado durante a execução do programa e precisa ser conhecido em tempo de compilação.
