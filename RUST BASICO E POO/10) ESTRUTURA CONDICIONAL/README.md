# ESTRUTURA CONDICIONAL
Em Rust, você pode usar as estruturas `if` e `else` para criar comandos de decisão e executar diferentes blocos de código com base em uma condição. A estrutura básica é a seguinte:

```rust
fn main() {
    let numero = 10;

    if numero > 5 {
        println!("O número é maior do que 5.");
    } else {
        println!("O número não é maior do que 5.");
    }
}
```

Neste exemplo simples, estamos usando um comando `if` para verificar se a variável `numero` é maior do que 5. Se a condição for verdadeira, o bloco de código dentro do `if` será executado. Caso contrário, o bloco de código dentro do `else` será executado.

Você também pode usar `else if` para testar várias condições em sequência:

```rust
fn main() {
    let numero = 10;

    if numero > 15 {
        println!("O número é maior do que 15.");
    } else if numero > 5 {
        println!("O número é maior do que 5, mas não maior do que 15.");
    } else {
        println!("O número não é maior do que 5.");
    }
}
```

Neste exemplo, adicionamos uma condição adicional com `else if`. O programa verifica as condições na ordem em que são apresentadas e executa o bloco de código do primeiro ramo que atende à condição.

Lembre-se de que em Rust, as condições em `if` e `else if` devem ser expressões booleanas, ou seja, elas devem avaliar para `true` ou `false`.

Você também pode usar operadores lógicos, como `&&` (e) e `||` (ou), para combinar condições em instruções `if` mais complexas.