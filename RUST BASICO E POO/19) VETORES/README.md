# CRIANDO VETORES
Em Rust, você pode criar vetores para armazenar uma coleção de valores do mesmo tipo. Os vetores são flexíveis e podem ser redimensionados dinamicamente à medida que você insere ou remove elementos. Aqui está como você pode criar e usar vetores em Rust:

1. **Criando um Vetor Vazio:**
   
   Você pode criar um vetor vazio especificando o tipo dos elementos que ele conterá:

   ```rust
   let mut vetor_vazio: Vec<i32> = Vec::new();
   ```

2. **Criando um Vetor com Elementos:**
   
   Você pode criar um vetor com elementos diretamente usando a macro `vec![]`:

   ```rust
   let vetor_com_elementos = vec![1, 2, 3, 4, 5];
   ```

3. **Adicionando Elementos a um Vetor:**
   
   Você pode adicionar elementos a um vetor usando o método `push`:

   ```rust
   let mut meu_vetor = Vec::new();
   meu_vetor.push(42);
   meu_vetor.push(123);
   ```

4. **Acessando Elementos de um Vetor:**
   
   Você pode acessar elementos de um vetor usando a notação de índice:

   ```rust
   let vetor = vec![10, 20, 30, 40];
   let primeiro_elemento = vetor[0]; // Acessa o primeiro elemento (índice 0)
   let segundo_elemento = vetor[1];  // Acessa o segundo elemento (índice 1)
   ```

   Tenha cuidado ao acessar elementos de um vetor para evitar índices inválidos, o que pode causar um pânico em tempo de execução. Você pode usar o método `get` para evitar panics:

   ```rust
   if let Some(elemento) = vetor.get(2) {
       println!("Terceiro elemento: {}", elemento);
   } else {
       println!("Índice inválido.");
   }
   ```

5. **Iterando sobre um Vetor:**
   
   Você pode iterar sobre os elementos de um vetor usando loops `for` ou métodos como `iter()`:

   ```rust
   let vetor = vec![10, 20, 30, 40];
   
   // Usando um loop for
   for elemento in &vetor {
       println!("Elemento: {}", elemento);
   }
   
   // Usando o método iter()
   for elemento in vetor.iter() {
       println!("Elemento: {}", elemento);
   }
   ```

6. **Removendo Elementos de um Vetor:**
   
   Você pode remover elementos de um vetor usando o método `remove` ou `pop`:

   ```rust
   let mut vetor = vec![1, 2, 3, 4, 5];
   
   // Remove o elemento no índice 2
   vetor.remove(2);
   
   // Remove e retorna o último elemento
   let elemento_removido = vetor.pop();
   ```

7. **Tamanho de um Vetor:**

   Você pode obter o tamanho de um vetor usando o método `len`:

   ```rust
   let vetor = vec![1, 2, 3, 4, 5];
   let tamanho = vetor.len();
   ```

Esses são os conceitos básicos para criar e trabalhar com vetores em Rust. Vetores são uma estrutura de dados muito útil para armazenar e manipular coleções de elementos.