# TIPOS DE DADOS
Rust oferece uma variedade de tipos de dados que podem ser usados para representar diferentes tipos de valores. Aqui estão alguns dos tipos de dados mais comuns em Rust:

1. **Tipos Integrais**:

   - `i8`, `i16`, `i32`, `i64`, `i128`: Números inteiros com sinal de 8, 16, 32, 64 ou 128 bits.
   - `u8`, `u16`, `u32`, `u64`, `u128`: Números inteiros sem sinal de 8, 16, 32, 64 ou 128 bits.
   - `isize` e `usize`: Inteiros com sinal e sem sinal que têm tamanho dependente da arquitetura do sistema.

2. **Tipos Ponto Flutuante**:

   - `f32`: Números de ponto flutuante de precisão simples.
   - `f64`: Números de ponto flutuante de precisão dupla (padrão).

3. **Tipo Booleano**:

   - `bool`: Representa um valor booleano, que pode ser `true` (verdadeiro) ou `false` (falso).

4. **Tipos de Caracteres**:

   - `char`: Representa um único caractere Unicode, denotado por aspas simples, como `'a'` ou `'🚀'`.

5. **Tipos de Tuplas**:

   - As tuplas são tipos compostos que podem conter um número fixo de valores de diferentes tipos. Por exemplo, `(i32, f64, char)` é uma tupla que pode conter um inteiro de 32 bits, um número de ponto flutuante de 64 bits e um caractere.

6. **Tipos de Vetores**:

   - Os vetores são coleções de elementos do mesmo tipo com tamanho fixo. Por exemplo, `[i32; 5]` representa um vetor de 5 inteiros de 32 bits.

7. **Tipos de Sequências Dinâmicas**:

   - `Vec<T>`: É um tipo genérico que representa um vetor dinâmico (ou lista) de elementos do tipo `T`.

8. **Tipo String**:

   - `String`: Representa uma sequência de caracteres Unicode de tamanho variável, sendo uma alocação de memória dinâmica.

9. **Tipos de Ponteiros Inteligentes**:

   - `Box<T>`: É um tipo que representa a propriedade exclusiva de um valor alocado no heap.
   - `Rc<T>`: Permite a criação de referências de contagem de referência para valores compartilhados imutáveis.
   - `Arc<T>`: Semelhante ao `Rc`, mas é thread-safe, permitindo compartilhamento seguro entre threads.

10. **Tipos de Funções**:

    - `fn`: Representa o tipo de uma função, permitindo que funções sejam atribuídas a variáveis ou passadas como argumentos para outras funções.

11. **Tipos de Enumerações (Enums)**:

    - `enum`: Permite a criação de tipos de dados personalizados, com várias variantes. Enums são frequentemente usados para representar escolhas ou estados em um programa.

12. **Tipos de Estruturas (Structs)**:

    - `struct`: Permite a criação de tipos de dados personalizados com campos nomeados.

13. **Tipos de Referências**:

    - `&T`: Representa uma referência imutável a um valor do tipo `T`.
    - `&mut T`: Representa uma referência mutável a um valor do tipo `T`.

14. **Tipo `Option<T>` e `Result<T, E>`**:

    - `Option<T>`: Representa um valor opcional que pode ser `Some(T)` (algum valor) ou `None` (nulo).
    - `Result<T, E>`: Representa um resultado que pode ser `Ok(T)` (operacao bem-sucedida) ou `Err(E)` (erro).

Esses são alguns dos tipos de dados fundamentais em Rust. Rust também suporta tipos genéricos, o que significa que você pode criar suas próprias estruturas de dados parametrizadas por tipos. Isso torna Rust uma linguagem flexível e expressiva para trabalhar com uma variedade de tipos de dados.