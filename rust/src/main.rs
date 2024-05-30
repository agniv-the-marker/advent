mod trebuchet;
mod conundrum;

const INPUT_DIRECTORY: &str = "../input/";

fn main() {
    println!("Solving trebuchet");
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", false).unwrap();
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", true).unwrap();
    println!();

    println!("Solving conundrum");
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", false).unwrap();
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", true).unwrap();
    // println!();
}
