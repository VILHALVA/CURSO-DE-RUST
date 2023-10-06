# ESTRUTURA DE REPETIÇÃO
Rust oferece três principais comandos de repetição para criar loops em seu código: `for`, `while` e `loop`. Vou explicar cada um deles com exemplos.

## `for` Loop:
O loop `for` é usado quando você sabe antecipadamente quantas vezes deseja repetir uma ação. Você normalmente usa um iterador ou um intervalo de números para controlar o número de iterações.

```rust
fn main() {
    // Loop for que imprime os números de 1 a 5
    for i in 1..=5 {
        println!("Número: {}", i);
    }
}
```

Neste exemplo, o loop `for` é usado para imprimir os números de 1 a 5. O intervalo `1..=5` é usado como iterador para controlar as iterações.

## `while` Loop:
O loop `while` é usado quando você deseja repetir uma ação enquanto uma condição for verdadeira.

```rust
fn main() {
    let mut contador = 1;

    // Loop while que imprime os números de 1 a 5
    while contador <= 5 {
        println!("Número: {}", contador);
        contador += 1;
    }
}
```

Neste exemplo, o loop `while` é usado para imprimir os números de 1 a 5 enquanto a condição `contador <= 5` for verdadeira. O contador é incrementado em cada iteração.

## `loop` Loop:
O loop `loop` é usado quando você deseja criar um loop infinito ou quando deseja sair do loop com base em alguma condição interna.

```rust
fn main() {
    let mut contador = 1;

    // Loop loop que imprime os números de 1 a 5 e sai quando o contador atinge 6
    loop {
        println!("Número: {}", contador);
        contador += 1;
        
        if contador > 5 {
            break; // Sai do loop quando o contador atinge 6
        }
    }
}
```

Neste exemplo, o loop `loop` é usado para imprimir os números de 1 a 5, mas o loop só sai quando o contador atinge 6, usando a instrução `break`.

Lembre-se de que você deve tomar cuidado ao usar loops infinitos (`loop`) para garantir que haja uma maneira de sair do loop, seja com uma condição interna ou com uma instrução `break`.