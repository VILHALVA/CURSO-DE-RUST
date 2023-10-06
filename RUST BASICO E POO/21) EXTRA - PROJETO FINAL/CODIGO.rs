trait CorpoCeleste {
    fn obter_nome(&self) -> &str;
    fn obter_raio(&self) -> f64;
    fn calcular_gravidade(&self) -> f64;
}

struct Estrela {
    nome: String,
    raio: f64,
}

impl Estrela {
    fn queimar(&self) {
        println!("A estrela {} está queimando!", self.nome);
    }
}

impl CorpoCeleste for Estrela {
    fn obter_nome(&self) -> &str {
        &self.nome
    }

    fn obter_raio(&self) -> f64 {
        self.raio
    }

    fn calcular_gravidade(&self) -> f64 {
        const G: f64 = 6.67430e-11;
        let massa = (4.0 / 3.0) * std::f64::consts::PI * self.raio.powi(3);
        G * (massa / self.raio.powi(2))
    }
}

struct Planeta {
    nome: String,
    raio: f64,
    distancia_media_do_sol: f64,
}

impl CorpoCeleste for Planeta {
    fn obter_nome(&self) -> &str {
        &self.nome
    }

    fn obter_raio(&self) -> f64 {
        self.raio
    }

    fn calcular_gravidade(&self) -> f64 {
        const G: f64 = 6.67430e-11;
        let massa = (4.0 / 3.0) * std::f64::consts::PI * self.raio.powi(3);
        G * (massa / self.raio.powi(2))
    }
}

fn exibir_informacoes_celestes<T: CorpoCeleste>(corpo_celeste: &T) {
    println!("Nome: {}", corpo_celeste.obter_nome());
    println!("Raio: {:.2} metros", corpo_celeste.obter_raio());
    println!("Gravidade: {:.2} m/s²", corpo_celeste.calcular_gravidade());
    println!();
}

fn main() {
    let sol = Estrela {
        nome: String::from("Sol"),
        raio: 696_340_000.0, // Em metros
    };

    let terra = Planeta {
        nome: String::from("Terra"),
        raio: 6_371_000.0,         // Em metros
        distancia_media_do_sol: 149_600_000_000.0, // Em metros
    };

    let marte = Planeta {
        nome: String::from("Marte"),
        raio: 3_389_500.0,         // Em metros
        distancia_media_do_sol: 227_940_000_000.0, // Em metros
    };

    exibir_informacoes_celestes(&sol);
    exibir_informacoes_celestes(&terra);
    exibir_informacoes_celestes(&marte);

    sol.queimar();
}
