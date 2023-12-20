use cmo::run;

fn main() {
    // load_config()
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}
