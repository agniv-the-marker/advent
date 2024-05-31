mod trebuchet;
mod conundrum;
mod gear;
mod scratchcard;
mod fertilizer;

const INPUT_DIRECTORY: &str = "../input/";

fn main() {
    println!("Solving trebuchet");
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", false).unwrap();
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", true).unwrap();

    println!("\nSolving conundrum");
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", false).unwrap();
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", true).unwrap();

    // Shift to make task 1 / task 2 run alongside each other for a single pass. More readable?

    println!("\nSolving gear ratios");
    gear::solve(INPUT_DIRECTORY, "gear.txt").unwrap();

    println!("\nSolving scratchcard");
    scratchcard::solve(INPUT_DIRECTORY, "scratchcard.txt").unwrap();

    println!("\nSolving fertilizer");
    fertilizer::solve(INPUT_DIRECTORY, "fertilizer.txt").unwrap();
}
