//! Input-only observations of the canonical raw EPW record/header owners.

#[path = "clk02_probe_support/mod.rs"]
mod support;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let request = arguments
        .next()
        .ok_or("usage: clk02_probe <input-only-request.json>")?;
    if arguments.next().is_some() {
        return Err("probe accepts exactly the input-only request".into());
    }
    let result = support::run(std::path::Path::new(&request))?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
