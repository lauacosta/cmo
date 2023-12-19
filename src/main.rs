use codex::run_codex;

fn main() {
    // load_config()
    if let Err(e) = run_codex() {
        eprintln!("Error: {e}");
    }

}
