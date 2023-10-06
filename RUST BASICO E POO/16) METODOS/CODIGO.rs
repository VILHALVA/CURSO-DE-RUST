// Definindo uma estrutura chamada "Casa"
struct Casa {
    area: f64,        // Área da casa em metros quadrados
    quartos: u32,     // Número de quartos na casa
    banheiros: u32,   // Número de banheiros na casa
    valor_por_metro_quadrado: f64,  // Valor por metro quadrado da região
}

// Implementando métodos para a estrutura "Casa"
impl Casa {
    // Método para calcular o custo total da casa
    fn calcular_custo_total(&self) -> f64 {
        let custo_area = self.area * self.valor_por_metro_quadrado;
        let custo_quartos = (self.quartos * 50000) as f64;  // Custo médio por quarto
        let custo_banheiros = (self.banheiros * 25000) as f64;  // Custo médio por banheiro
        custo_area + custo_quartos + custo_banheiros
    }
}

fn main() {
    // Criando uma instância da estrutura "Casa"
    let casa1 = Casa {
        area: 150.0,
        quartos: 3,
        banheiros: 2,
        valor_por_metro_quadrado: 2000.0,  // Valor médio por metro quadrado
    };

    // Chamando o método "calcular_custo_total" para calcular o custo da casa
    let custo_total = casa1.calcular_custo_total();

    // Exibindo o custo total da casa
    println!("Custo Total da Casa: R$ {:.2}", custo_total);
}
