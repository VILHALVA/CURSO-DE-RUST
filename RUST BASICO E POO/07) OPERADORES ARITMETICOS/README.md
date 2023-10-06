# OPERADORES ARITMETICOS
Em Rust, você pode usar os operadores aritméticos padrão para realizar operações matemáticas em números. Aqui estão os operadores aritméticos disponíveis em Rust:

1. **Adição (`+`)**: Realiza a adição de dois valores.

   ```rust
   let resultado = 5 + 3; // resultado contém 8
   ```

2. **Subtração (`-`)**: Realiza a subtração de dois valores.

   ```rust
   let resultado = 10 - 4; // resultado contém 6
   ```

3. **Multiplicação (`*`)**: Realiza a multiplicação de dois valores.

   ```rust
   let resultado = 3 * 7; // resultado contém 21
   ```

4. **Divisão (`/`)**: Realiza a divisão de dois valores.

   ```rust
   let resultado = 10 / 2; // resultado contém 5
   ```

5. **Módulo (`%`)**: Retorna o resto da divisão de dois valores.

   ```rust
   let resultado = 10 % 3; // resultado contém 1 (resto da divisão de 10 por 3)
   ```

6. **Incremento (`+=`)**: Adiciona um valor a uma variável existente.

   ```rust
   let mut numero = 5;
   numero += 3; // Agora, numero contém 8
   ```

7. **Decremento (`-=`)**: Subtrai um valor de uma variável existente.

   ```rust
   let mut numero = 10;
   numero -= 4; // Agora, numero contém 6
   ```

8. **Operadores Unários**:
   - **Negativo (`-`)**: Inverte o sinal de um número.

     ```rust
     let numero = 5;
     let resultado = -numero; // resultado contém -5
     ```

   - **Incremento Prefixo (`++`)** e **Decremento Prefixo (`--`)**: Rust não suporta esses operadores. Você deve usar as formas de incremento e decremento explicadas acima.

Lembre-se de que os operadores aritméticos seguem as regras padrão da matemática e podem ser usados com números inteiros e de ponto flutuante em Rust. Certifique-se de que os tipos de dados sejam compatíveis ao usar operadores aritméticos, pois Rust é uma linguagem fortemente tipada e não faz coerção automática de tipos.