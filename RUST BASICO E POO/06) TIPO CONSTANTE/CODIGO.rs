// Constantes para configurações do aplicativo
const VERSAO: &str = "1.0.0";
const AUTOR: &str = "Alice";
const LIMITE_DE_USUARIOS: u32 = 1000;

// Função que verifica se um usuário está ativo
fn verificar_status_usuario(usuario: &str) -> bool {
    // Simulação: Verificar o status do usuário em um banco de dados
    let usuarios_ativos = ["Alice", "Bob", "Charlie", "David"];
    usuarios_ativos.contains(&usuario)
}

fn main() {
    println!("Bem-vindo ao Meu App (v{})!", VERSAO);
    println!("Autor: {}", AUTOR);
    println!("Limite de usuários: {}", LIMITE_DE_USUARIOS);

    let usuario = "Alice";
    if verificar_status_usuario(usuario) {
        println!("O usuário {} está ativo.", usuario);
    } else {
        println!("O usuário {} não está ativo.", usuario);
    }

    // Tente alterar o valor da constante (isso resultará em um erro de compilação):
    // VERSAO = "2.0.0"; // Erro: não é possível atribuir a uma constante após a definição
}
