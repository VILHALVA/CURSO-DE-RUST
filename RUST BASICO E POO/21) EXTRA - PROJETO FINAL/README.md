# PROJETO FINAL - DESAFIO 
Vamos criar um desafio que envolve a criação de um sistema solar simples em Rust, usando os conceitos de Programação Orientada a Objetos (POO), encapsulamento, abstração, herança (usando traits) e polimorfismo. Neste desafio, vamos modelar planetas e estrelas.

**Desafio: Sistema Solar em Rust**

Você deve criar um programa Rust que modele um sistema solar com planetas e uma estrela (o sol). Cada planeta deve ter nome, raio e distância média do sol. A estrela (sol) também deve ter um nome e raio.

1. Crie uma estrutura chamada `Estrela` que representa uma estrela com nome e raio. Implemente um método chamado `queimar()` que exibe uma mensagem indicando que a estrela está queimando.

2. Crie um trait chamado `CorpoCeleste` que contenha os seguintes métodos:

   - `obter_nome(&self) -> &str`: Retorna o nome do corpo celeste.
   - `obter_raio(&self) -> f64`: Retorna o raio do corpo celeste.
   - `calcular_gravidade(&self) -> f64`: Calcula a gravidade do corpo celeste com base no raio e na distância média do sol. A fórmula é `gravidade = G * (massa / raio^2)`, onde `G` é a constante gravitacional (use um valor fixo, por exemplo, 6.67430e-11), `massa = 4/3 * π * raio^3` (assuma que todos os corpos celestes têm densidade uniforme) e `raio` é o raio do corpo celeste.

3. Crie estruturas para representar três planetas diferentes com nome, raio e distância média do sol. Cada planeta deve implementar o trait `CorpoCeleste`.

4. Crie uma função chamada `exibir_informacoes_celestes<T: CorpoCeleste>(corpo_celeste: T)` que aceita qualquer corpo celeste que implemente o trait `CorpoCeleste` e exibe informações sobre o corpo celeste, incluindo nome, raio e gravidade.

5. No `main()`, crie instâncias da estrela e dos planetas, exiba informações sobre eles usando a função `exibir_informacoes_celestes`, e chame o método `queimar()` da estrela.

Este desafio combina todos os conceitos que aprendemos até agora, incluindo a criação de estruturas, implementação de traits, herança (por meio de traits) e polimorfismo. Ele permite que você modele um sistema solar simples em Rust e calcule a gravidade de cada corpo celeste com base em suas características.