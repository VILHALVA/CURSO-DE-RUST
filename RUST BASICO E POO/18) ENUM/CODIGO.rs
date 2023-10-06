// Definindo um enum chamado "AnimalEstimacao"
enum AnimalEstimacao {
    Gato,
    Cachorro,
}

// Definindo uma estrutura para representar um animal de estimação
struct Pet {
    nome: String,
    tipo: AnimalEstimacao,
    idade: u8,
}

fn main() {
    // Criando instâncias de animais de estimação
    let gato1 = Pet {
        nome: String::from("Whiskers"),
        tipo: AnimalEstimacao::Gato,
        idade: 3,
    };

    let cachorro1 = Pet {
        nome: String::from("Rex"),
        tipo: AnimalEstimacao::Cachorro,
        idade: 5,
    };

    // Função para descrever um animal de estimação
    fn descrever_pet(pet: &Pet) -> String {
        match pet.tipo {
            AnimalEstimacao::Gato => format!("Nome: {}, Gato de {} anos", pet.nome, pet.idade),
            AnimalEstimacao::Cachorro => format!("Nome: {}, Cachorro de {} anos", pet.nome, pet.idade),
        }
    }

    // Exibindo informações dos animais de estimação
    println!("{}", descrever_pet(&gato1));
    println!("{}", descrever_pet(&cachorro1));
}
