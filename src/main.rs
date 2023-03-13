fn main() {
    let portugues: f32 = 8.0;
    let matematica: f32 = 6.0;
    let historia: f32 = 7.5;
    let mut media: f32 = 0.0;


    println!("Português: {portugues}");
    println!("Matemática: {matematica}");
    println!("História: {historia}\n");

    println!("Valor da média antes do cálculo: {media}\n");

    media = (portugues + matematica + historia) / 3.0;
    println!("Valor da média entre português, matemática e história: {media}\n");

    media = (portugues + matematica) / 2.0;
    println!("Valor da média entre português e matemática: {media}");

    
}
