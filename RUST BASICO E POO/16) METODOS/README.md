# CRIANDO METODOS
Em Rust, você pode criar métodos para tipos personalizados (structs, enums e traits) através de implementações. Para criar um método para uma estrutura (struct), você deve usar a palavra-chave `impl` para implementar um bloco de código associado a essa estrutura. Vou mostrar como criar um método para uma estrutura em Rust.

Aqui está um exemplo de como criar um método `descrever` para uma estrutura chamada `Pessoa`:

```rust
// Definindo uma estrutura chamada "Pessoa"
struct Pessoa {
    nome: String,
    idade: u32,
}

// Implementando métodos para a estrutura "Pessoa"
impl Pessoa {
    // Método para descrever a pessoa
    fn descrever(&self) -> String {
        format!("Nome: {}, Idade: {} anos", self.nome, self.idade)
    }
}

fn main() {
    // Criando uma instância da estrutura "Pessoa"
    let pessoa = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };

    // Chamando o método "descrever" para exibir informações sobre a pessoa
    let descricao = pessoa.descrever();
    println!("{}", descricao);
}
```

Neste exemplo:

1. Definimos uma estrutura `Pessoa` com dois campos: `nome` e `idade`.

2. Usamos a palavra-chave `impl` para implementar métodos para a estrutura `Pessoa`.

3. Criamos um método `descrever` que aceita `&self` como parâmetro (o que significa que é uma referência imutável à própria instância) e retorna uma string que descreve a pessoa com base em seus campos.

4. No `main()`, criamos uma instância da estrutura `Pessoa` chamada `pessoa` e chamamos o método `descrever()` para obter uma descrição da pessoa. Em seguida, exibimos a descrição.

Este é um exemplo simples de como criar e usar métodos em Rust. Métodos permitem que você associe comportamentos específicos a tipos personalizados, tornando seu código mais organizado e modular.