# CLASSE POO
Os quatro pilares da Programação Orientada a Objetos (POO) são conceitos fundamentais que ajudam a organizar e estruturar o código em sistemas baseados em objetos. Vou explicar cada um desses pilares e fornecer exemplos relacionados a Rust:

1. **Encapsulamento:**
   
   O encapsulamento é o conceito de agrupar dados (atributos) e comportamentos (métodos) em uma única unidade chamada de classe ou tipo. O encapsulamento permite que você controle o acesso aos atributos e métodos, tornando alguns deles públicos (acessíveis de fora da classe) e outros privados (acessíveis apenas de dentro da classe).

   Exemplo em Rust:
   
   ```rust
   struct Pessoa {
       nome: String,
       idade: u32,
   }

   impl Pessoa {
       fn new(nome: String, idade: u32) -> Self {
           Pessoa { nome, idade }
       }

       fn obter_nome(&self) -> &str {
           &self.nome
       }

       fn definir_idade(&mut self, nova_idade: u32) {
           self.idade = nova_idade;
       }
   }
   ```

   Neste exemplo, a estrutura `Pessoa` encapsula os atributos `nome` e `idade`, bem como os métodos `new`, `obter_nome` e `definir_idade`.

2. **Abstração:**
   
   A abstração é o processo de simplificar objetos do mundo real em representações mais simples em código. Isso envolve a identificação dos atributos e comportamentos essenciais de um objeto e a criação de uma classe ou tipo correspondente.

   Exemplo em Rust:

   ```rust
   struct Retângulo {
       largura: f64,
       altura: f64,
   }

   impl Retângulo {
       fn calcular_area(&self) -> f64 {
           self.largura * self.altura
       }
   }
   ```

   Neste exemplo, a estrutura `Retângulo` representa um objeto do mundo real (um retângulo) e encapsula seus atributos (`largura` e `altura`). O método `calcular_area` é uma operação abstrata que pode ser realizada em um retângulo.

3. **Herança:**
   
   A herança é um mecanismo que permite criar uma nova classe (subclasse ou classe derivada) com base em uma classe existente (superclasse ou classe base). A subclasse herda os atributos e métodos da superclasse e pode adicionar ou substituir comportamentos específicos.

   Em Rust, a herança é parcialmente substituída pelo conceito de "composição" e "traits" (traços), que permitem a reutilização de código de maneira mais flexível. Rust não suporta herança de classes tradicionais como algumas outras linguagens de programação.

   Exemplo de uso de traits em Rust (substituto da herança):

   ```rust
   trait Animal {
       fn fazer_som(&self);
   }

   struct Cachorro;
   struct Gato;

   impl Animal for Cachorro {
       fn fazer_som(&self) {
           println!("O cachorro faz um latido.");
       }
   }

   impl Animal for Gato {
       fn fazer_som(&self) {
           println!("O gato faz um miado.");
       }
   }
   ```

   Neste exemplo, criamos um trait `Animal` com um método `fazer_som`, e as estruturas `Cachorro` e `Gato` implementam o trait. Isso permite que objetos do tipo `Cachorro` e `Gato` compartilhem a mesma interface.

4. **Polimorfismo:**
   
   O polimorfismo permite que objetos de diferentes tipos sejam tratados de maneira uniforme, desde que eles implementem uma interface comum. Isso permite que você escreva código que funcione com objetos de várias classes ou tipos diferentes sem se preocupar com os detalhes específicos de cada um.

   Exemplo em Rust:

   ```rust
   fn emitir_som(animal: &dyn Animal) {
       animal.fazer_som();
   }

   let cachorro = Cachorro;
   let gato = Gato;

   emitir_som(&cachorro);
   emitir_som(&gato);
   ```

   Neste exemplo, a função `emitir_som` aceita qualquer objeto que implemente o trait `Animal`. Isso permite que chamemos a função com um objeto `Cachorro` ou `Gato`, demonstrando o polimorfismo.

Em Rust, o polimorfismo é frequentemente alcançado por meio do uso de traits, como mostrado no exemplo acima. Os traits permitem que você escreva código genérico que pode ser aplicado a objetos de diferentes tipos que compartilham a mesma interface.

Embora Rust adote uma abordagem diferente para alguns desses pilares em comparação com linguagens de programação orientada a objetos mais tradicionais, como C++ ou Java, os conceitos fundamentais da POO ainda podem ser aplicados para criar código organiz