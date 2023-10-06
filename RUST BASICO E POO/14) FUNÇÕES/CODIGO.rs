// Função para calcular a média de um vetor de números
fn calcular_media(valores: &[f64]) -> f64 {
    let soma: f64 = valores.iter().sum();
    let media = soma / valores.len() as f64;
    media
}

// Função para calcular a mediana de um vetor de números
fn calcular_mediana(valores: &mut Vec<f64>) -> f64 {
    valores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let meio = valores.len() / 2;
    
    if valores.len() % 2 == 0 {
        (valores[meio - 1] + valores[meio]) / 2.0
    } else {
        valores[meio]
    }
}

// Função para calcular a moda de um vetor de números
fn calcular_moda(valores: &[f64]) -> f64 {
    use std::collections::HashMap;

    let mut contagem = HashMap::new();
    for &valor in valores {
        *contagem.entry(valor).or_insert(0) += 1;
    }

    let (moda, _) = contagem.iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    **moda
}

fn main() {
    let numeros = vec![5.0, 3.5, 8.0, 3.5, 2.5, 8.0, 5.0, 9.0];
    
    // Calcula e exibe a média
    let media = calcular_media(&numeros);
    println!("Média: {:.2}", media);
    
    // Calcula e exibe a mediana
    let mut numeros_copia = numeros.clone();
    let mediana = calcular_mediana(&mut numeros_copia);
    println!("Mediana: {:.2}", mediana);

    // Calcula e exibe a moda
    let moda = calcular_moda(&numeros);
    println!("Moda: {:.2}", moda);
}
