use std::time::Instant;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    dex_builder::compile("assets/pokedex/pokemon", "assets/pokedex/moves", "assets/pokedex/items", "assets/pokedex/trainers", Some("output/dex.bin"), false);
    println!("Finished in {}ms!", start.elapsed().as_millis());
}