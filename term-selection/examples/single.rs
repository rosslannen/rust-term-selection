use term_selection::TermSelection;

#[derive(Clone, Copy, TermSelection, Debug)]
#[prompt = "Choose a color"]
enum ColorSelection {
    #[description = "A blue color"]
    Blue,
    Red,
    #[description = "Lets gooooooo"]
    Teal,
}

fn main() {
    let color_selection = ColorSelection::get_single_selection();
    println!("Single: {:?}", color_selection);
}
