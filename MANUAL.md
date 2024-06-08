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

## 2. CONFIGURAÇÃO DA IDE (INTEGRATED DEVELOPMENT ENVIRONMENT):
Usar uma IDE facilita muito o desenvolvimento em Rust. Uma das IDEs mais populares para Rust é o Visual Studio Code.

### VISUAL STUDIO CODE:
1. Baixe e instale o Visual Studio Code no [site oficial](https://code.visualstudio.com/).
2. Abra o Visual Studio Code.
3. Instale a extensão Rust (rls):
   1. Clique no ícone de Extensões no lado esquerdo.
   2. Pesquise por "rust" e instale a extensão oficial do Rust (rls).

## 3. CRIANDO O PRIMEIRO PROJETO EM RUST:
### PASSO A PASSO PARA CRIAR UM PROJETO NO VISUAL STUDIO CODE:
1. Abra o terminal.
2. Navegue até o diretório onde deseja criar o projeto.
3. Execute o comando:
   ```sh
   cargo new hello_world
   ```
4. Navegue até o diretório do projeto:
   ```sh
   cd hello_world
   ```

### CRIANDO UM ARQUIVO RUST:
1. Abra o Visual Studio Code.
2. Clique em "File" > "Open Folder" e selecione a pasta `hello_world`.
3. No painel do lado esquerdo, expanda a pasta `src` e abra o arquivo `main.rs`.

### ESCREVENDO O CÓDIGO:
No arquivo `main.rs`, escreva o seguinte código:
```rust
fn main() {
    println!("Hello, World!");
}
```

### EXECUTANDO O PROJETO:
1. Abra o terminal integrado no Visual Studio Code (View > Terminal).
2. No terminal, certifique-se de que está no diretório do projeto (`hello_world`).
3. Execute o comando:
   ```sh
   cargo run
   ```

Você verá a mensagem `Hello, World!` impressa no terminal.

## CONCLUSÃO:
Agora você tem o Rust instalado e configurado, além de um ambiente de desenvolvimento Rust pronto com o Visual Studio Code. Você criou e executou seu primeiro projeto Rust. A partir daqui, você pode explorar mais sobre a linguagem Rust, bibliotecas e frameworks para expandir suas habilidades de programação.