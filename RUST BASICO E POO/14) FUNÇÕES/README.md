# FUNÇÕES:
Em Rust, as funções são blocos de código reutilizáveis que realizam uma tarefa específica. Elas são uma parte fundamental da estrutura de qualquer programa Rust. Aqui está uma visão geral das funções em Rust:

1. **Declaração de Função:** Para declarar uma função em Rust, use a palavra-chave `fn`, seguida do nome da função e da lista de parâmetros. A lista de parâmetros é uma série de variáveis que a função espera receber como entrada.

2. **Corpo da Função:** O corpo da função é um bloco de código delimitado por chaves `{}`. Este é o lugar onde você escreve o código que a função executará quando for chamada.

3. **Parâmetros:** As funções podem aceitar parâmetros, que são valores que podem ser passados para a função quando ela é chamada. Os parâmetros são listados entre parênteses após o nome da função. Os parâmetros são tipados, o que significa que você deve especificar o tipo de cada parâmetro.

4. **Valor de Retorno:** As funções em Rust podem ter um tipo de valor de retorno, que é especificado após uma seta `->` e é usado para indicar o tipo de dado que a função retornará.

Aqui está um exemplo simples de uma função em Rust que adiciona dois números e retorna o resultado:

```rust
fn soma(a: i32, b: i32) -> i32 {
    let resultado = a + b;
    resultado // Valor de retorno implícito
}

fn main() {
    let num1 = 5;
    let num2 = 7;

    let resultado_da_soma = soma(num1, num2);
    println!("Resultado da Soma: {}", resultado_da_soma);
}
```

Neste exemplo, temos uma função chamada `soma` que aceita dois parâmetros `a` e `b`, realiza a soma e retorna o resultado como um valor de tipo `i32` (inteiro de 32 bits). No `main()`, chamamos essa função com os valores 5 e 7 e exibimos o resultado.

É importante notar que Rust tem uma semântica de propriedade exclusiva (ownership) para passagem de parâmetros por padrão, o que significa que a função `soma` recebe a propriedade dos valores `num1` e `num2`. Se você quiser que a função apenas faça referência aos valores sem possuí-los, você pode usar referências (usando `&`) ou copiar os valores (usando `Clone`).

Funções são uma parte essencial da programação em Rust e ajudam a organizar o código de forma modular e reutilizável.