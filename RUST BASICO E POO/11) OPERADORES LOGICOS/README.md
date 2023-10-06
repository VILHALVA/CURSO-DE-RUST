# OPERADORES LOGICOS
Rust oferece operadores lógicos padrão que você pode usar para combinar ou negar expressões booleanas. Aqui estão os operadores lógicos disponíveis em Rust:

1. **E lógico (`&&`)**: Este operador retorna verdadeiro (`true`) se ambas as expressões booleanas à esquerda e à direita forem verdadeiras.

   ```rust
   let a = true;
   let b = false;

   let resultado = a && b; // Resultado é falso (false)
   ```

2. **OU lógico (`||`)**: Este operador retorna verdadeiro (`true`) se pelo menos uma das expressões booleanas à esquerda ou à direita for verdadeira.

   ```rust
   let a = true;
   let b = false;

   let resultado = a || b; // Resultado é verdadeiro (true)
   ```

3. **Negação lógica (`!`)**: Este operador inverte o valor de uma expressão booleana.

   ```rust
   let a = true;

   let resultado = !a; // Resultado é falso (false)
   ```

Aqui está um exemplo que demonstra o uso de operadores lógicos em Rust:

```rust
fn main() {
    let a = true;
    let b = false;

    // Operador E lógico
    let resultado_e = a && b;
    println!("a && b: {}", resultado_e);

    // Operador OU lógico
    let resultado_ou = a || b;
    println!("a || b: {}", resultado_ou);

    // Negação lógica
    let resultado_negacao = !a;
    println!("!a: {}", resultado_negacao);
}
```

Neste exemplo, demonstramos o uso de todos os três operadores lógicos em Rust. Lembre-se de que os operadores lógicos são frequentemente usados para combinar condições em instruções `if` e `else if`, permitindo tomar decisões com base em expressões booleanas compostas.