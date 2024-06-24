# MANUAL
## 1. INSTALAÇÃO DO RUST
Rust é uma linguagem de programação que requer a instalação do compilador `rustc` e do gerenciador de pacotes `cargo`.

### WINDOWS:
1. Baixe o instalador do Rust acessando o [site oficial do Rust](https://www.rust-lang.org/tools/install).
2. Clique em "Download rustup-init.exe".
3. Execute o instalador e siga as instruções na tela.
4. Após a instalação, abra o Prompt de Comando e execute:
   ```sh
   rustc --version
   ```
   e
   ```sh
   cargo --version
   ```
   para verificar se a instalação foi bem-sucedida.

### MACOS:
1. Abra o Terminal.
2. Execute o seguinte comando para instalar o Rust usando o `rustup`:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Siga as instruções na tela para completar a instalação.
4. Após a instalação, reinicie o terminal e execute:
   ```sh
   rustc --version
   ```
   e
   ```sh
   cargo --version
   ```
   para verificar se a instalação foi bem-sucedida.

### LINUX:
1. Abra um terminal.
2. Execute o seguinte comando para instalar o Rust usando o `rustup`:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Siga as instruções na tela para completar a instalação.
4. Após a instalação, reinicie o terminal e execute:
   ```sh
   rustc --version
   ```
   e
   ```sh
   cargo --version
   ```
   para verificar se a instalação foi bem-sucedida.


## 2. CRIAR UM NOVO PROJETO:
Para criar um novo projeto Rust, você pode usar o comando `cargo new`:

```sh
cargo new hello_world
cd hello_world
```

## 3. ESCREVER CÓDIGO RUST:
No arquivo `src/main.rs`, você pode escrever o seguinte código básico:

```rust
fn main() {
    println!("Hello, world!");
}
```

## 4. COMPILAR E EXECUTAR:
Dentro do diretório do projeto, compile e execute seu programa com os seguintes comandos:

```sh
cargo build
cargo run
```

## CONCLUSÃO:
Agora você tem o Rust instalado e configurado, além de um ambiente de desenvolvimento Rust pronto com o Visual Studio Code. Você criou e executou seu primeiro projeto Rust. A partir daqui, você pode explorar mais sobre a linguagem Rust, bibliotecas e frameworks para expandir suas habilidades de programação.