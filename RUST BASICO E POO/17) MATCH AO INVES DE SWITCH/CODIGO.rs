// Definindo um enum chamado "Fruta" com diferentes variantes
enum Fruta {
    Maca,
    Banana,
    Laranja,
    Morango,
}

fn main() {
    let fruta = Fruta::Banana;

    match fruta {
        Fruta::Maca => println!("Você escolheu uma maçã."),
        Fruta::Banana => println!("Você escolheu uma banana."),
        Fruta::Laranja => println!("Você escolheu uma laranja."),
        Fruta::Morango => println!("Você escolheu um morango."),
    }
}
