// Trait que define um veículo
trait Veiculo {
    fn ligar(&self);
    fn acelerar(&self);
    fn parar(&self);
}

// Estrutura que representa um carro
struct Carro {
    marca: String,
    modelo: String,
}

// Implementação do trait Veiculo para Carro
impl Veiculo for Carro {
    fn ligar(&self) {
        println!("{} {} ligado.", self.marca, self.modelo);
    }

    fn acelerar(&self) {
        println!("{} {} acelerando.", self.marca, self.modelo);
    }

    fn parar(&self) {
        println!("{} {} parou.", self.marca, self.modelo);
    }
}

// Estrutura que representa uma bicicleta
struct Bicicleta {
    marca: String,
    estilo: String,
}

// Implementação do trait Veiculo para Bicicleta
impl Veiculo for Bicicleta {
    fn ligar(&self) {
        println!("{} {} não precisa ser ligada.", self.marca, self.estilo);
    }

    fn acelerar(&self) {
        println!("{} {} pedalando mais rápido.", self.marca, self.estilo);
    }

    fn parar(&self) {
        println!("{} {} parou pedalando.", self.marca, self.estilo);
    }
}

// Função genérica que aceita qualquer veículo que implementa Veiculo
fn testar_veiculo<T: Veiculo>(veiculo: T) {
    veiculo.ligar();
    veiculo.acelerar();
    veiculo.parar();
}

fn main() {
    let carro = Carro {
        marca: String::from("Toyota"),
        modelo: String::from("Corolla"),
    };

    let bicicleta = Bicicleta {
        marca: String::from("Trek"),
        estilo: String::from("Mountain Bike"),
    };

    println!("Testando Carro:");
    testar_veiculo(carro);

    println!("\nTestando Bicicleta:");
    testar_veiculo(bicicleta);
}
