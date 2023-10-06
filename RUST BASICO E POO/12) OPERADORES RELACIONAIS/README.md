# OPERADORES RELACIONAIS
Em Rust, os operadores relacionais são usados para comparar valores e produzir resultados booleanos (verdadeiro ou falso) que são usados em estruturas de decisão e em lógica condicional. Aqui estão os operadores relacionais disponíveis em Rust:

1. **Igual (`==`)**: Verifica se dois valores são iguais.

   ```rust
   let a = 5;
   let b = 5;
   let igual = a == b; // true
   ```

2. **Diferente (`!=`)**: Verifica se dois valores são diferentes.

   ```rust
   let a = 5;
   let b = 10;
   let diferente = a != b; // true
   ```

3. **Maior (`>`)**: Verifica se um valor é maior do que o outro.

   ```rust
   let a = 10;
   let b = 5;
   let maior = a > b; // true
   ```

4. **Maior ou Igual (`>=`)**: Verifica se um valor é maior ou igual ao outro.

   ```rust
   let a = 10;
   let b = 10;
   let maior_ou_igual = a >= b; // true
   ```

5. **Menor (`<`)**: Verifica se um valor é menor do que o outro.

   ```rust
   let a = 5;
   let b = 10;
   let menor = a < b; // true
   ```

6. **Menor ou Igual (`<=`)**: Verifica se um valor é menor ou igual ao outro.

   ```rust
   let a = 5;
   let b = 5;
   let menor_ou_igual = a <= b; // true
   ```

Aqui está um exemplo que demonstra o uso de operadores relacionais em Rust:

```rust
fn main() {
    let numero1 = 10;
    let numero2 = 5;

    // Igualdade
    let igualdade = numero1 == numero2;
    println!("Igualdade: {}", igualdade);

    // Diferença
    let diferenca = numero1 != numero2;
    println!("Diferença: {}", diferenca);

    // Maior
    let maior = numero1 > numero2;
    println!("Maior: {}", maior);

    // Maior ou Igual
    let maior_ou_igual = numero1 >= numero2;
    println!("Maior ou Igual: {}", maior_ou_igual);

    // Menor
    let menor = numero1 < numero2;
    println!("Menor: {}", menor);

    // Menor ou Igual
    let menor_ou_igual = numero1 <= numero2;
    println!("Menor ou Igual: {}", menor_ou_igual);
}
```

Neste exemplo, usamos operadores relacionais para comparar valores numéricos e obter resultados booleanos. Esses resultados booleanos podem ser usados em estruturas de decisão, como instruções `if`, para tomar decisões com base nas relações entre os valores.