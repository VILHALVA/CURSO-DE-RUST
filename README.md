# CURSO DE RUST
👨‍⚖️RUST É UMA LINGUAGEM DE PROGRAMAÇÃO.

[![GitHub Repo stars](https://img.shields.io/badge/VILHALVA-GITHUB-03A9F4?logo=github)](https://github.com/VILHALVA) 
[![GitHub Repo stars](https://img.shields.io/badge/VEJA-DOCUMENTAÇÃO-03A9F4?logo=google)](https://prev.rust-lang.org/pt-BR/documentation.html) 
[![GitHub Repo stars](https://img.shields.io/badge/-PLAYLIST%20DO%20YOUTUBE-blueviolet)](https://www.youtube.com/playlist?list=PLGtFJAmtESz-V5p-svTX34bGQvNuXEpHE)

# 👀VISÃO PANORÂMICA:
| PERGUNTA | RESPOSTA |
| :---: | :---: |
| DATA DE CRIAÇÃO | 2010 |
| NOME DO CRIADOR | Graydon Hoare | 
| SIGNIFICADO DO NOME | O nome "Rust" é uma referência ao óxido de ferro, que é conhecido por ser resistente à ferrugem. |
| É BASEADA NO | C++, Haskell e OCaml |
| EXTENÇÃO DO ARQUIVO | .rs |
| É MAIS USADA | Sistemas embarcados |

- Data de criação: Rust foi criada em 2010, com o desenvolvimento inicial liderado por Graydon Hoare. A primeira versão estável, Rust 1.0, foi lançada em maio de 2015.

- Criador: Rust foi desenvolvida principalmente por Graydon Hoare, que trabalhava na Mozilla na época do projeto. No entanto, o desenvolvimento posterior envolveu uma comunidade de colaboradores ativos.

- Significado do nome: O nome "Rust" é uma referência ao óxido de ferro, que é conhecido por ser resistente à ferrugem. O nome simboliza a ênfase da linguagem na segurança e na prevenção de erros de programação.

- Baseada em: Rust não é diretamente baseada em nenhuma outra linguagem, mas foi influenciada por várias linguagens, incluindo C++, Haskell e OCaml.

- Extensão do arquivo: Os arquivos de código-fonte Rust geralmente têm a extensão ".rs". Os projetos Rust também podem incluir arquivos de configuração com a extensão "Cargo.toml", referentes ao sistema de gerenciamento de pacotes Cargo.

- Uso principal: Rust é conhecida por sua ênfase em segurança, concorrência e desempenho. Ela é frequentemente usada para o desenvolvimento de sistemas de baixo nível, como sistemas operacionais, programas embarcados e software de infraestrutura de servidores. Além disso, Rust tem ganhado popularidade no desenvolvimento de aplicativos de alto desempenho e seguros, como navegadores web (por exemplo, Firefox), servidores web e sistemas de jogos.

- Mascote/Logo: Rust não tem um mascote oficial ou uma logo estabelecida como algumas outras linguagens, como o gopher do Go ou o mascote do Python. A comunidade Rust geralmente usa o logotipo oficial da linguagem, que consiste em uma estilizada "R" e uma folha, representando o óxido de ferro (ferrugem). O caranguejo é frequentemente usado de forma humorística e não oficial para se referir a Rust, devido à semelhança fonética entre "Rust" e "crustáceo" (crab em inglês). Esta associação surgiu espontaneamente dentro da comunidade Rust e acabou se tornando um símbolo não oficial da linguagem. No entanto, é importante observar que o caranguejo não é um mascote oficial reconhecido pela equipe de desenvolvimento da Rust, mas é uma parte divertida da cultura da comunidade.

# 🤳SINTAXE DA LINGUAGEM:
## 0) FUNDAMENTOS:
Aqui está um exemplo de código Rust que utiliza operadores aritméticos, relacionais e lógicos com tipos primitivos:

```rust
fn main() {
    // Variáveis com valores iniciais
    let a = 10;
    let b = 5;

    // Operadores aritméticos
    let soma = a + b;
    let subtracao = a - b;
    let multiplicacao = a * b;
    let divisao = a / b;
    let resto = a % b;

    println!("Operadores aritméticos:");
    println!("Soma: {}", soma);
    println!("Subtração: {}", subtracao);
    println!("Multiplicação: {}", multiplicacao);
    println!("Divisão: {}", divisao);
    println!("Resto: {}", resto);

    // Operadores relacionais
    let igual = a == b;
    let diferente = a != b;
    let maior = a > b;
    let menor = a < b;
    let maior_ou_igual = a >= b;
    let menor_ou_igual = a <= b;

    println!("\nOperadores relacionais:");
    println!("Igual: {}", igual);
    println!("Diferente: {}", diferente);
    println!("Maior: {}", maior);
    println!("Menor: {}", menor);
    println!("Maior ou igual: {}", maior_ou_igual);
    println!("Menor ou igual: {}", menor_ou_igual);

    // Operadores lógicos
    let x = true;
    let y = false;

    let and = x && y;
    let or = x || y;
    let not = !x;

    println!("\nOperadores lógicos:");
    println!("AND: {}", and);
    println!("OR: {}", or);
    println!("NOT: {}", not);
}
```

Neste exemplo, definimos variáveis `a` e `b` com valores inteiros, realizamos operações aritméticas como soma, subtração, multiplicação, divisão e cálculo do resto, e em seguida, usamos operadores relacionais para comparar esses valores. Também demonstramos o uso de operadores lógicos com variáveis booleanas `x` e `y`. O programa imprime os resultados dessas operações.

## 1) VARIAVEIS SIMPLES:
Em Rust, você pode criar variáveis simples usando a palavra-chave `let`, seguida pelo nome da variável e, opcionalmente, seu tipo. Aqui estão alguns exemplos de declarações de variáveis simples em Rust:

```rust
fn main() {
    // Declaração de uma variável inteira (i32) com valor inicial 42
    let numero = 42;

    // Declaração de uma variável de ponto flutuante (f64) com valor inicial 3.14
    let pi = 3.14;

    // Declaração de uma variável booleana (bool) com valor inicial true
    let esta_chovendo = true;

    // Declaração de uma variável de caractere (char) com valor inicial 'A'
    let letra = 'A';

    // Declaração de uma variável de cadeia de caracteres (String) com valor inicial "Olá, Rust!"
    let mensagem = String::from("Olá, Rust!");

    println!("Número: {}", numero);
    println!("Pi: {}", pi);
    println!("Está chovendo? {}", esta_chovendo);
    println!("Letra: {}", letra);
    println!("Mensagem: {}", mensagem);
}
```

Neste exemplo, declaramos variáveis simples com diferentes tipos de dados, como inteiros, números de ponto flutuante, booleanos, caracteres e cadeias de caracteres. Observe que, em Rust, o tipo das variáveis pode ser inferido automaticamente com base no valor inicial, mas você também pode especificar o tipo explicitamente usando `let nome: tipo = valor`.

Além disso, note que as variáveis em Rust são, por padrão, imutáveis. Isso significa que, após atribuir um valor a uma variável, você não pode alterá-la. Se desejar que a variável seja mutável, você deve usar a palavra-chave `mut`, como em `let mut numero = 42;`, para indicar que a variável pode ser modificada posteriormente.

Para obter dados de entrada do usuário em Rust, você pode usar a biblioteca padrão `std::io`. Aqui está um exemplo simples de como ler uma linha de entrada do usuário:

```rust
use std::io;

fn main() {
    println!("Digite algo:");

    let mut input = String::new(); // Crie uma variável para armazenar a entrada

    // Use a função `stdin` para obter uma referência ao fluxo de entrada padrão
    // e, em seguida, chame `read_line` para ler uma linha da entrada do usuário
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

    // Agora, você pode usar o valor lido em `input`
    println!("Você digitou: {}", input);
}
```

Neste exemplo, começamos importando a biblioteca `std::io` para poder usar suas funções de entrada/saída. Em seguida, criamos uma variável chamada `input` do tipo `String` para armazenar a entrada do usuário.

Usamos `io::stdin().read_line(&mut input)` para ler uma linha da entrada padrão (teclado) e armazená-la na variável `input`. O método `expect` é usado para lidar com erros que podem ocorrer durante a leitura da linha.

Por fim, imprimimos a entrada do usuário para confirmar que a leitura foi bem-sucedida.

Você pode executar este código e digitar algo quando solicitado, e o programa exibirá o que você digitou.

## 2) ESTRUTURA CONDICIONAL:
### ESTRUTURA IF-ELSE:
A estrutura de controle `if-else` em Rust permite que você execute diferentes blocos de código com base em uma condição. Aqui está um exemplo simples de como usar `if-else` em Rust:

```rust
fn main() {
    let numero = 42;

    if numero > 50 {
        println!("O número é maior do que 50");
    } else if numero == 50 {
        println!("O número é igual a 50");
    } else {
        println!("O número é menor do que 50");
    }
}
```

Neste exemplo, usamos a variável `numero` e testamos três condições diferentes:

1. Se `numero` for maior do que 50, ele imprimirá "O número é maior do que 50".
2. Se `numero` for igual a 50, ele imprimirá "O número é igual a 50".
3. Se nenhuma das condições anteriores for verdadeira (ou seja, `numero` for menor do que 50), ele imprimirá "O número é menor do que 50".

Você pode ter quantos blocos `else if` forem necessários e um bloco `else` é opcional. Apenas o bloco correspondente à primeira condição verdadeira será executado.

Lembre-se de que em Rust, as condições dentro de `if` e `else if` devem ser expressões booleanas (ou seja, avaliadas como `true` ou `false`). A lógica subjacente à estrutura `if-else` é a mesma em muitas linguagens de programação, facilitando a compreensão e o uso.

Em Rust, o `else` não é obrigatório estar na mesma linha do fechamento de bloco do `if`. A formatação e a indentação em Rust não são tão rígidas quanto em algumas outras linguagens de programação. Você pode formatar o código de maneira que o `else` esteja em uma linha separada, desde que siga as regras básicas de sintaxe. Aqui está um exemplo válido:

```rust
fn main() {
    let numero = 42;

    if numero > 50 {
        println!("O número é maior do que 50");
    } 
    else if numero == 50 {
        println!("O número é igual a 50");
    } 
    else {
        println!("O número é menor do que 50");
    }
}
```

O código acima é perfeitamente válido em Rust. O importante é que a estrutura do `if`, `else if`, e `else` esteja correta em termos de aninhamento e indentação, tornando o código legível e seguindo a lógica de controle de fluxo desejada.

### ESTRUTURA IF-ELSE, ELSE IF:
A estrutura de controle `if-else if-else` em Rust permite que você execute diferentes blocos de código com base em várias condições. Aqui está um exemplo de como usá-la:

```rust
fn main() {
    let numero = 42;

    if numero > 50 {
        println!("O número é maior do que 50");
    } else if numero == 50 {
        println!("O número é igual a 50");
    } else if numero < 50 {
        println!("O número é menor do que 50");
    } else {
        println!("O número é inválido");
    }
}
```

Neste exemplo, usamos a variável `numero` e testamos várias condições:

1. Se `numero` for maior do que 50, ele imprimirá "O número é maior do que 50".
2. Se `numero` for igual a 50, ele imprimirá "O número é igual a 50".
3. Se `numero` for menor do que 50, ele imprimirá "O número é menor do que 50".
4. Se nenhuma das condições anteriores for verdadeira, ele imprimirá "O número é inválido".

Você pode ter quantos blocos `else if` forem necessários para verificar várias condições, seguidos por um bloco `else` opcional que lida com qualquer caso que não foi coberto pelas condições anteriores. Apenas o bloco correspondente à primeira condição verdadeira será executado.

### ESTRUTURA SWITCH:
Rust não possui uma estrutura de controle `switch` como algumas outras linguagens de programação, como C/C++, Java ou JavaScript. Em vez disso, Rust oferece uma alternativa mais poderosa chamada `match`. A expressão `match` permite que você compare um valor com várias opções e execute um bloco de código correspondente à opção correspondente. Veja como você pode usar `match` em Rust:

```rust
fn main() {
    let escolha = 2;

    match escolha {
        1 => println!("Você escolheu a opção 1"),
        2 => println!("Você escolheu a opção 2"),
        3 => println!("Você escolheu a opção 3"),
        _ => println!("Opção inválida"), // O sublinhado (_) é um curinga para qualquer outra escolha
    }
}
```

Neste exemplo, usamos `match` para comparar o valor da variável `escolha` com várias opções. Se o valor de `escolha` corresponder a uma das opções, o bloco de código correspondente será executado. Caso contrário, o bloco com `_` será executado, representando um valor padrão ou inválido.

O uso do `_` como um curinga é opcional, mas pode ser útil para lidar com todos os casos não especificados explicitamente.

Embora Rust não tenha um `switch`, a expressão `match` é mais flexível e segura, pois força a consideração de todos os casos possíveis e evita bugs comuns, como esquecer de incluir um `break` em uma cláusula `case`, o que é comum em outras linguagens com `switch`.

## 3) ESTRUTURA DE REPETIÇÃO:
### ESTRUTURA FOR:
Em Rust, a estrutura de repetição `for` é usada para iterar por uma sequência de valores, como um intervalo de números, elementos de uma coleção ou outros tipos de sequências. O `for` em Rust é frequentemente usado em conjunto com iteradores para percorrer coleções, como vetores ou fatias. Aqui está um exemplo básico de como usar o `for` em Rust:

```rust
fn main() {
    // Iterando por um intervalo de números de 1 a 5 (inclusive)
    for numero in 1..=5 {
        println!("Número: {}", numero);
    }

    // Iterando por uma coleção (neste caso, um vetor)
    let numeros = vec![10, 20, 30, 40, 50];
    for numero in &numeros {
        println!("Número no vetor: {}", numero);
    }
}
```

Neste exemplo:

1. Usamos o `for` para iterar por um intervalo de números de 1 a 5, incluindo 5 (`1..=5`). Cada número é atribuído à variável `numero`, e imprimimos seu valor.

2. Em seguida, usamos o `for` para iterar por um vetor chamado `numeros`. O operador `&` é usado para criar uma referência ao vetor, uma vez que o `for` requer uma referência para evitar a propriedade do vetor. O valor de cada elemento do vetor é atribuído à variável `numero`, e imprimimos esses valores.

O `for` em Rust é uma ferramenta poderosa para percorrer coleções e iterar sobre elementos em uma variedade de cenários. Você pode personalizar a iteração usando iteradores e até mesmo criar seus próprios iteradores personalizados, se necessário.

### ESTRUTURA WHILE:
A estrutura de repetição `while` em Rust permite que você execute um bloco de código repetidamente enquanto uma condição específica for verdadeira. Aqui está um exemplo de como usar o `while` em Rust:

```rust
fn main() {
    let mut contador = 0;

    while contador < 5 {
        println!("Contador: {}", contador);
        contador += 1;
    }
}
```

Neste exemplo:

1. Inicializamos uma variável mutável `contador` com o valor inicial de 0.
2. Usamos o `while` para criar um loop que executa enquanto a condição `contador < 5` for verdadeira.
3. Dentro do loop, imprimimos o valor atual do contador.
4. Incrementamos o contador com `contador += 1` para que ele eventualmente se torne igual ou maior que 5, encerrando o loop.

O `while` é útil quando você não sabe o número exato de iterações necessárias com antecedência e deseja que o loop continue até que uma condição específica seja atendida. Lembre-se de tomar cuidado para evitar loops infinitos, certificando-se de que a condição de parada seja alcançada em algum momento.

### ESTRUTURA DO-WHILE:
Rust não possui uma estrutura de repetição `do-while` como algumas outras linguagens, como C/C++. No entanto, você pode simular um loop `do-while` usando um loop `while` com uma condição de parada no final do bloco. Aqui está um exemplo de como criar um equivalente a um loop `do-while` em Rust:

```rust
fn main() {
    let mut contador = 0;

    loop {
        println!("Contador: {}", contador);
        contador += 1;

        if contador >= 5 {
            break;
        }
    }
}
```

Neste exemplo, estamos usando um loop `loop`, que é um loop infinito por padrão, e usamos uma instrução `if` com `break` para sair do loop quando a condição desejada for atendida. O resultado é semelhante ao comportamento de um `do-while` em outras linguagens:

1. O loop `loop` é iniciado, que sempre executa pelo menos uma vez.
2. Dentro do loop, imprimimos o valor atual do contador e incrementamos o contador.
3. Usamos a instrução `if` para verificar se o contador é maior ou igual a 5 e, se for, usamos `break` para sair do loop.

Dessa forma, garantimos que o bloco de código dentro do loop seja executado pelo menos uma vez antes de verificar a condição de saída. Isso simula o comportamento de um loop `do-while`.

## 4) VARIAVEIS COMPOSTAS:
Em Rust, existem duas categorias principais de variáveis compostas: tipos de coleção e tipos de estrutura. Abaixo estão exemplos e explicações dos tipos de variáveis compostas mais comuns em Rust:

### Vetores (Arrays):
   Os vetores são coleções de elementos do mesmo tipo, com um tamanho fixo que não pode ser alterado após a sua criação.

   Exemplo:
   ```rust
   let numeros: [i32; 4] = [1, 2, 3, 4];
   ```

   Neste exemplo, criamos um vetor de 4 elementos inteiros de 32 bits.

## Fatiamento (Slices):
   Os fatiamentos são uma visualização de parte de um vetor. Eles permitem acessar uma parte específica de um vetor sem copiar os dados.

   Exemplo:
   ```rust
   let numeros = [1, 2, 3, 4, 5];
   let fatia = &numeros[1..4]; // Fatiando o vetor para obter [2, 3, 4]
   ```

## Vetores Dinâmicos:
   Os vetores dinâmicos são coleções redimensionáveis que podem crescer ou encolher dinamicamente.

   Exemplo:
   ```rust
   let mut numeros = Vec::new();
   numeros.push(1);
   numeros.push(2);
   ```

   Neste exemplo, `numeros` é um vetor dinâmico que começa vazio e pode crescer conforme adicionamos elementos.

### Tuplas:
   Tuplas são coleções ordenadas de elementos que podem ter tipos diferentes. São fixas em tamanho, mas flexíveis em relação aos tipos de dados que podem conter.

   Exemplo:
   ```rust
   let pessoa: (String, i32) = ("Alice".to_string(), 30);
   ```

   Neste exemplo, `pessoa` é uma tupla que contém uma string e um inteiro.

## Estruturas (Structs):
   Estruturas permitem que você crie tipos de dados personalizados definindo a estrutura dos dados que eles armazenarão.

   Exemplo:
   ```rust
   struct Pessoa {
       nome: String,
       idade: i32,
   }

   let alice = Pessoa {
       nome: "Alice".to_string(),
       idade: 30,
   };
   ```

   Neste exemplo, definimos uma estrutura `Pessoa` com campos `nome` e `idade` e criamos uma instância dela chamada `alice`.

### Enumerações (Enums):
   Enums permitem criar tipos de dados que podem assumir um de vários valores discretos.

   Exemplo:
   ```rust
   enum DiaDaSemana {
       Segunda,
       Terça,
       Quarta,
       Quinta,
       Sexta,
       Sábado,
       Domingo,
   }

   let hoje = DiaDaSemana::Terça;
   ```

   Neste exemplo, definimos uma enumeração `DiaDaSemana` com diferentes variantes representando os dias da semana e atribuímos a variável `hoje` o valor `Terça`.

Esses são alguns dos tipos de variáveis compostas mais comuns em Rust. Eles são essenciais para lidar com dados estruturados e organizados em programas Rust.

## 5) FUNÇÕES:
Em Rust, as funções são blocos de código que realizam uma tarefa específica e podem ser chamadas de outros lugares do programa. As funções desempenham um papel fundamental na organização e modularização de código. Aqui estão os principais elementos de funções em Rust:

1. **Declaração de Função**:
   
   Para declarar uma função em Rust, você usa a palavra-chave `fn`, seguida pelo nome da função e uma lista de parâmetros entre parênteses. A declaração da função termina com um bloco de código delimitado por chaves `{}` que contém o código da função.

   ```rust
   fn saudacao(nome: &str) {
       println!("Olá, {}!", nome);
   }
   ```

   Neste exemplo, declaramos uma função chamada `saudacao` que recebe um parâmetro `nome` do tipo `&str` e imprime uma saudação.

2. **Chamada de Função**:

   Para chamar uma função em Rust, você simplesmente usa seu nome seguido por argumentos entre parênteses.

   ```rust
   saudacao("Alice");
   ```

   Esta chamada de função chama a função `saudacao` com o argumento `"Alice"`.

3. **Retorno de Função**:

   Funções em Rust podem ter um valor de retorno especificado após uma seta `->`. O valor de retorno é o resultado da função.

   ```rust
   fn soma(a: i32, b: i32) -> i32 {
       a + b
   }
   ```

   Neste exemplo, a função `soma` recebe dois números inteiros `a` e `b` como parâmetros e retorna sua soma como um valor `i32`.

4. **Funções com ou sem Efeito Colateral**:

   Em Rust, as funções podem ser definidas como "com efeito colateral" ou "sem efeito colateral". Funções com efeito colateral geralmente realizam ações que afetam o ambiente externo, como imprimir na tela ou modificar variáveis globais. Funções sem efeito colateral apenas calculam e retornam valores sem interagir com o ambiente externo.

5. **Funções Aninhadas**:

   Rust permite a criação de funções aninhadas, ou seja, funções dentro de outras funções. As funções aninhadas podem acessar variáveis da função pai.

   ```rust
   fn outer_function(x: i32) {
       fn inner_function(y: i32) {
           println!("Valor de y: {}", y);
       }

       inner_function(x);
   }
   ```

   Neste exemplo, a função `inner_function` está aninhada dentro da função `outer_function` e pode acessar o parâmetro `x` da função pai.

6. **Funções Genéricas**:

   Rust suporta funções genéricas que podem aceitar argumentos de diferentes tipos. Isso permite criar funções flexíveis e reutilizáveis.

   ```rust
   fn troca<T>(a: T, b: T) -> (T, T) {
       (b, a)
   }
   ```

   Neste exemplo, a função `troca` é genérica e pode ser usada para trocar os valores de variáveis de qualquer tipo compatível.

As funções desempenham um papel crucial na organização de código em Rust, permitindo que você divida a lógica do programa em unidades reutilizáveis e compreensíveis. Elas também contribuem para a segurança e a expressividade da linguagem.

## 6) CLASS POO:
A Programação Orientada a Objetos (POO) é uma abordagem de programação que se baseia em quatro pilares fundamentais: Encapsulamento, Abstração, Herança e Polimorfismo. Em Rust, embora a linguagem não siga o paradigma POO de maneira pura como algumas outras linguagens (como C++ ou Java), você pode usar conceitos semelhantes. Aqui estão exemplos de código Rust que ilustram cada um dos quatro pilares da POO:

1. **Encapsulamento**:

   O encapsulamento é a ideia de que os detalhes internos de um objeto devem ser ocultos do mundo exterior e acessados apenas por meio de interfaces controladas.

   ```rust
   struct ContaBancaria {
       saldo: f64,
   }

   impl ContaBancaria {
       fn novo(saldo_inicial: f64) -> Self {
           ContaBancaria { saldo: saldo_inicial }
       }

       fn depositar(&mut self, valor: f64) {
           self.saldo += valor;
       }

       fn sacar(&mut self, valor: f64) {
           if valor <= self.saldo {
               self.saldo -= valor;
           } else {
               println!("Saldo insuficiente.");
           }
       }

       fn obter_saldo(&self) -> f64 {
           self.saldo
       }
   }

   fn main() {
       let mut minha_conta = ContaBancaria::novo(100.0);
       minha_conta.depositar(50.0);
       minha_conta.sacar(30.0);

       println!("Saldo: {:.2}", minha_conta.obter_saldo());
   }
   ```

   Neste exemplo, a estrutura `ContaBancaria` encapsula o saldo e fornece métodos para interagir com ele, garantindo que o saldo não seja acessado diretamente.

2. **Abstração**:

   Abstração é o processo de simplificar objetos complexos, fornecendo uma interface clara e ocultando os detalhes de implementação.

   ```rust
   trait Animal {
       fn fazer_som(&self);
   }

   struct Cachorro;
   struct Gato;

   impl Animal for Cachorro {
       fn fazer_som(&self) {
           println!("O cachorro faz au au!");
       }
   }

   impl Animal for Gato {
       fn fazer_som(&self) {
           println!("O gato faz miau!");
       }
   }

   fn main() {
       let meu_cachorro = Cachorro;
       let meu_gato = Gato;

       meu_cachorro.fazer_som();
       meu_gato.fazer_som();
   }
   ```

   Neste exemplo, usamos uma trait chamada `Animal` para abstrair o conceito de fazer som. As estruturas `Cachorro` e `Gato` implementam essa trait, fornecendo uma implementação concreta para `fazer_som`.

3. **Herança**:

   Em Rust, você pode usar composição para alcançar reutilização de código em vez de herança tradicional. No entanto, você ainda pode criar estruturas que compartilham comportamento com o uso de composição.

   ```rust
   struct Veiculo {
       velocidade: f64,
   }

   impl Veiculo {
       fn new(velocidade: f64) -> Self {
           Veiculo { velocidade }
       }

       fn acelerar(&self) {
           println!("O veículo acelera a {} km/h.", self.velocidade);
       }
   }

   struct Carro {
       veiculo: Veiculo,
   }

   impl Carro {
       fn new(velocidade: f64) -> Self {
           Carro {
               veiculo: Veiculo::new(velocidade),
           }
       }

       fn dirigir(&self) {
           println!("O carro está em movimento.");
           self.veiculo.acelerar();
       }
   }

   fn main() {
       let meu_carro = Carro::new(120.0);
       meu_carro.dirigir();
   }
   ```

   Neste exemplo, usamos composição para criar uma relação de "tem um" entre `Carro` e `Veiculo`. O carro possui um veículo e herda seu comportamento.

4. **Polimorfismo**:

   O polimorfismo permite que objetos de diferentes tipos sejam tratados de maneira uniforme. Em Rust, o polimorfismo é frequentemente alcançado usando traits.

   ```rust
   trait Forma {
       fn area(&self) -> f64;
   }

   struct Retangulo {
       largura: f64,
       altura: f64,
   }

   struct Circulo {
       raio: f64,
   }

   impl Forma for Retangulo {
       fn area(&self) -> f64 {
           self.largura * self.altura
       }
   }

   impl Forma for Circulo {
       fn area(&self) -> f64 {
           std::f64::consts::PI * self.raio * self.raio
       }
   }

   fn calcular_area<T: Forma>(forma: &T) -> f64 {
       forma.area()
   }

   fn main() {
       let retangulo = Retangulo {
           largura: 5.0,
           altura: 3.0,
       };
       let circulo = Circulo { raio: 2.0 };

       println!("Área do retângulo: {:.2}", calcular_area(&retangulo));
       println!("Área do círculo: {:.2}", calcular_area(&circulo));
   }
   ```

   Neste exemplo, usamos uma trait chamada `Forma` para definir um método `area()`. As estruturas `Retangulo` e `Circulo` implementam essa trait, permitindo que ambas as formas sejam tratadas de maneira polimórfica usando a função `calcular_area()`.

# 💖CARACTERISTICAS DA LINGUAGEM:
## ❤POSITIVAS:
1. **Segurança de Memória**: Rust é conhecida por seu sistema de propriedade de empréstimo (ownership) e sistema de tipos que previne erros de acesso à memória, como null pointers e buffer overflows, em tempo de compilação. Isso torna o código Rust altamente seguro e ajuda a evitar muitos bugs comuns.

2. **Sem Gerenciamento de Memória**: Rust oferece controle preciso sobre o gerenciamento de memória, permitindo que você aloque e desaloque manualmente memória quando necessário. No entanto, isso é feito de forma segura usando empréstimos, referências e o sistema de tipos.

3. **Ausência de Garbage Collection**: Rust não possui um coletor de lixo (garbage collector), o que significa que não há pausas inesperadas no programa para liberar memória. Isso o torna adequado para sistemas com restrições de desempenho e tempo real.

4. **Concorrência Segura**: Rust possui um modelo de concorrência baseado em propriedades que evita problemas comuns de concorrência, como race conditions. O sistema de tipos de Rust garante que os dados sejam acessados de forma segura por várias threads.

5. **Eficaz e Eficiente**: Rust foi projetada para oferecer um alto desempenho, permitindo o controle fino sobre os recursos do sistema. Isso a torna adequada para sistemas embarcados, servidores e outras aplicações de alto desempenho.

6. **Ferramentas de Compilação Avançadas**: O compilador Rust é conhecido por fornecer mensagens de erro detalhadas e úteis, facilitando a depuração de código. Além disso, o ecossistema Rust oferece ferramentas como o Cargo para gerenciar dependências e facilitar o desenvolvimento.

7. **Abordagem Funcional**: Rust suporta programação funcional e permite a criação de código mais limpo e expressivo, com características como iteradores e closures.

8. **Comunidade Ativa e Suporte**: Rust possui uma comunidade de desenvolvedores ativa e crescente, com documentação abrangente, fóruns e bibliotecas de terceiros. Além disso, é mantida por organizações como a Mozilla, o que garante um compromisso de longo prazo com seu desenvolvimento.

9. **Multiplataforma**: Rust é compatível com várias plataformas, incluindo Windows, macOS, Linux e outros sistemas operacionais. Também suporta compilação cruzada para diferentes arquiteturas.

10. **Linguagem Expressiva e Moderna**: Rust oferece uma sintaxe limpa e expressiva, inspirada em linguagens como C++, mas com melhorias significativas. Ela incorpora conceitos modernos de programação e possui características como pattern matching.

## 🖤NEGATIVAS:
1. **Curva de Aprendizado**: Rust pode ser uma linguagem desafiadora para aprender, especialmente para programadores que não têm experiência anterior em programação de sistemas ou em linguagens com sistemas de tipos complexos. A ênfase na segurança e no controle de memória pode tornar o código inicial mais complicado.

2. **Complexidade**: A segurança e o controle de memória oferecidos por Rust podem exigir código adicional e detalhado em comparação com linguagens de programação de alto nível, o que pode aumentar a complexidade do código.

3. **Gerenciamento de Memória Manual**: Embora o gerenciamento de memória manual seja uma vantagem para muitos, também pode ser visto como uma desvantagem, já que requer uma atenção cuidadosa para evitar erros de alocação e desalocação de memória.

4. **Tempo de Compilação**: O compilador Rust é conhecido por ser altamente rigoroso e, às vezes, o tempo de compilação pode ser mais longo do que em outras linguagens. Isso pode afetar o ciclo de desenvolvimento rápido.

5. **Falta de Ecossistema Maduro**: Embora Rust tenha uma comunidade crescente, o ecossistema de bibliotecas e frameworks ainda não é tão vasto ou maduro quanto em linguagens mais estabelecidas. Isso pode requerer mais esforço ao lidar com necessidades específicas de um projeto.

6. **Ausência de Garbage Collector**: Enquanto a ausência de um coletor de lixo é uma vantagem para muitos, pode ser uma desvantagem para desenvolvedores que preferem a comodidade do gerenciamento automático de memória.

7. **Compatibilidade Binária**: Devido às preocupações com segurança e estabilidade, a compatibilidade binária entre diferentes versões de bibliotecas Rust pode ser desafiadora, resultando em problemas de compatibilidade em projetos grandes.

8. **Documentação em Desenvolvimento**: Algumas bibliotecas e recursos podem ter documentação limitada ou em desenvolvimento, tornando-o menos amigável para desenvolvedores que dependem de uma documentação completa.

9. **Restrições de Tamanho de Binários**: Em sistemas embarcados e outros cenários de recursos limitados, os binários gerados por Rust podem ser maiores do que os de linguagens de programação de mais baixo nível.

10. **Acesso a Baixo Nível**: Enquanto Rust fornece acesso a baixo nível, algumas operações de baixo nível podem ser mais verbosas e complexas em comparação com linguagens de programação puramente de baixo nível, como C ou Assembly.

