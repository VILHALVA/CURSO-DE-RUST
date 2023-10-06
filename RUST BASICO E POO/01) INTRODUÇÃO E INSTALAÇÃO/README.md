# Introdução a Rust
Rust é uma linguagem de programação moderna, de alto desempenho e segura. Ela foi projetada para fornecer controle próximo ao hardware, segurança de memória e concorrência segura, enquanto ainda permite a expressividade e a facilidade de uso de linguagens de alto nível. Rust é frequentemente usada para desenvolver sistemas de software, aplicativos de alto desempenho, servidores e muito mais.

Algumas características notáveis de Rust incluem:

- **Segurança de memória**: Rust possui um sistema de propriedade de empréstimo (ownership) e sistema de tipos que previne erros de acesso à memória, como null pointers e buffer overflows, em tempo de compilação.

- **Concorrência segura**: Rust possui um modelo de concorrência baseado em propriedades que ajuda a evitar problemas comuns de concorrência, como race conditions.

- **Controle preciso sobre memória**: Você pode gerenciar manualmente a alocação e desalocação de memória em Rust, sem a necessidade de um coletor de lixo.

- **Performance de alto nível**: Rust é projetada para oferecer alto desempenho e eficiência, tornando-a adequada para sistemas com restrições de desempenho.

- **Ecossistema ativo**: Rust possui uma comunidade de desenvolvedores ativa e um conjunto crescente de bibliotecas e ferramentas para desenvolvimento.

# Instalação e Configuração do `rustup`
Para começar a programar em Rust, você precisará instalar o Rust e configurar seu ambiente de desenvolvimento. O `rustup` é a ferramenta oficial para instalação e gerenciamento de Rust em diferentes plataformas. Siga as etapas abaixo para instalar o Rust usando o `rustup`:

## 1. Instale o `rustup`:
Primeiro, abra um terminal (ou prompt de comando) e siga as instruções específicas para o seu sistema operacional:

- **Linux ou macOS**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Windows**:
  Baixe e execute o instalador `rustup-init.exe` a partir do site oficial: [Rustup for Windows](https://rustup.rs/).

## 2. Siga as instruções de instalação:
O `rustup` irá guiá-lo pelo processo de instalação. Durante a instalação, você pode optar por instalar o compilador `rustc`, a ferramenta `cargo` (gerenciador de pacotes e construção) e outros componentes relacionados. Normalmente, você deve escolher a opção padrão pressionando `Enter` para instalar tudo.

## 3. Verifique a instalação:
Após a instalação, abra um novo terminal e execute o seguinte comando para verificar se o Rust foi instalado corretamente:

```bash
rustc --version
```

Você também pode verificar a versão do Cargo, que é o gerenciador de pacotes e construção:

```bash
cargo --version
```

Isso deve exibir a versão atual do Rust e do Cargo.

## 4. Configuração do ambiente:
O `rustup` geralmente configura automaticamente o ambiente para que você possa começar a usar Rust. Não é necessário configurar variáveis de ambiente manualmente.

Agora você está pronto para começar a escrever e compilar programas em Rust! Você pode usar um editor de código de sua escolha, como VSCode, Sublime Text, ou até mesmo o próprio Rust Playground (https://play.rust-lang.org/) para testar código Rust diretamente no navegador. Lembre-se de consultar a documentação oficial de Rust (https://doc.rust-lang.org/book/) e os recursos online para aprender mais sobre a linguagem.