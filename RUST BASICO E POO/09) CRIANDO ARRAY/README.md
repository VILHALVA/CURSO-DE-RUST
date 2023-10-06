# CRIANDO ARRAY
Em Rust, você pode criar arrays usando a sintaxe `[T; N]`, onde `T` é o tipo dos elementos do array e `N` é o tamanho fixo do array. Os arrays em Rust têm tamanho fixo em tempo de compilação, o que significa que você deve especificar o número exato de elementos que o array conterá.

Aqui está um exemplo de como criar e inicializar um array em Rust:

```rust
fn main() {
    // Criando um array de inteiros com tamanho 5
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];

    // Acessando elementos do array usando índices (índice baseado em zero)
    let primeiro_numero = numeros[0];
    let segundo_numero = numeros[1];

    println!("Primeiro número: {}", primeiro_numero);
    println!("Segundo número: {}", segundo_numero);

    // Você também pode criar um array preenchido com um valor inicial
    let zeros: [i32; 3] = [0; 3]; // Cria um array com três zeros: [0, 0, 0]

    println!("Zeros: {:?}", zeros);
}
```

Neste exemplo, criamos um array `numeros` de tamanho 5 que contém os primeiros cinco números inteiros. Você pode acessar os elementos do array usando índices, que são baseados em zero. Além disso, mostramos como criar um array preenchido com um valor inicial, no caso, um array `zeros` com três elementos, todos iguais a zero.

Lembre-se de que os arrays em Rust têm tamanho fixo e não podem ser redimensionados após a criação. Se você precisar de uma coleção de tamanho dinâmico, considere usar um vetor (`Vec`) em vez de um array.