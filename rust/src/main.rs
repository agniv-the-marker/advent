mod trebuchet;
mod conundrum;
mod gear;
mod scratchcard;

const INPUT_DIRECTORY: &str = "../input/";

fn main() {
    println!("Solving trebuchet");
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", false).unwrap();
    trebuchet::solve(INPUT_DIRECTORY, "trebuchet.txt", true).unwrap();
    println!();

    println!("Solving conundrum");
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", false).unwrap();
    conundrum::solve(INPUT_DIRECTORY, "conundrum.txt", true).unwrap();
    println!();

    // Shift to make task 1 / task 2 run alongside each other for a single pass. More readable?

    println!("Solving gear ratios");
    gear::solve(INPUT_DIRECTORY, "gear.txt").unwrap();
    println!();

    println!("Solving scratchcard");
    scratchcard::solve(INPUT_DIRECTORY, "scratchcard.txt").unwrap();
    println!();
}
