use rust_learn::utils::email::{build_mock_verification_email, verification_url};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args
        .first()
        .is_some_and(|arg| arg == "--help" || arg == "-h")
    {
        println!("Usage: cargo run --bin mock_email --features tool-bin -- [email] [name] [token]");
        return;
    }

    let to = args
        .first()
        .map(String::as_str)
        .unwrap_or("learner@example.com");
    let name = args.get(1).map(String::as_str).unwrap_or("Demo Learner");
    let token = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("mock-preview-token");

    let verification_url = verification_url(token);
    println!(
        "{}",
        build_mock_verification_email(to, name, &verification_url)
    );
}
