# CURSO DE RUST
👨‍⚖️RUST É UMA LINGUAGEM DE PROGRAMAÇÃO.

<img src="FOTO.png" align="center" width="400"> <br>

## VISÃO PANORÂMICA:
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

## SUA HISTÓRIA:
A história do Rust remonta ao início dos anos 2000, quando a Mozilla Foundation estava procurando por uma nova linguagem de programação que fosse segura, concorrente e prática para o desenvolvimento de sistemas e aplicativos de alto desempenho. O desenvolvimento do Rust foi iniciado por Graydon Hoare, um engenheiro da Mozilla, em 2006, como um projeto paralelo enquanto trabalhava em outros projetos.

O objetivo inicial do Rust era criar uma linguagem de programação que fosse segura, concorrente e eficiente em termos de memória, enquanto também fosse prática e fácil de usar para os desenvolvedores. A equipe de desenvolvimento tinha como meta resolver problemas comuns encontrados em outras linguagens de programação, como falhas de segurança devido a erros de memória, dificuldades no desenvolvimento de software concorrente e falta de abstração de baixo nível.

O desenvolvimento do Rust continuou ao longo dos anos, com várias iterações e experimentos para refinar a linguagem e suas características. Em 2010, a Mozilla anunciou oficialmente o projeto Rust ao público e, em 2012, o projeto foi reescrito a partir do zero com uma nova sintaxe e um novo conjunto de recursos.

A primeira versão estável do Rust, Rust 1.0, foi lançada em maio de 2015. Este lançamento marcou um marco importante na evolução da linguagem, sinalizando que o Rust estava pronto para uso em produção e tinha uma base sólida de recursos e funcionalidades.

Desde então, o Rust ganhou popularidade rapidamente, especialmente entre desenvolvedores que valorizam a segurança, o desempenho e a concorrência em seus projetos. O Rust é amplamente adotado em uma variedade de áreas, incluindo desenvolvimento de sistemas, desenvolvimento de jogos, infraestrutura de software e desenvolvimento de aplicativos web.

Algumas das características distintivas do Rust incluem seu sistema de tipos forte e seguro, seu modelo de concorrência baseado em atores, sua sintaxe expressiva e sua eficiência em termos de memória. O Rust também possui uma comunidade ativa de desenvolvedores e uma biblioteca de ecossistema crescente, incluindo ferramentas de desenvolvimento, bibliotecas de código aberto e frameworks.

Atualmente, o Rust continua a evoluir com novas versões e atualizações regulares, introduzindo novos recursos, melhorias de desempenho e refinamentos na linguagem. O futuro do Rust parece promissor, com um número crescente de empresas e organizações adotando a linguagem e investindo em seu desenvolvimento e ecossistema.

## CARACTERISTICAS:
### POSITIVAS:
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

### NEGATIVAS:
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

## SUBSIDIOS:
- [CURSO CRIADO PELO "LINGUAGEM RUST"](https://www.youtube.com/playlist?list=PLGtFJAmtESz-V5p-svTX34bGQvNuXEpHE)
- [CURSO FEITO PELO VILHALVA](https://github.com/VILHALVA)
- [VEJA A DOCUMENTAÇÃO](https://prev.rust-lang.org/pt-BR/documentation.html)
- [VEJA A SINTAXE](./SINTAXE.md)
- [VEJA OS PROJETOS](https://github.com/VILHALVA?tab=repositories&q=topic:RUST)
