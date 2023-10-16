fn parse_url(input: &str) -> Option<url::Url> {
    if let Ok(url) = url::Url::parse(input) {
        Some(url)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let file_content = match std::fs::read_to_string("src/data/content.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!(err);
            std::process::exit(1);
        }
    };
    */

    let file_content = std::fs::read_to_string("src/data/content.txt")?;

    //let line_count = file_content.lines().count();

    /*
    let non_empty_lines: Vec<_> = file_content.lines()
        .filter(|line| !line.is_empty())
        .collect();

    println!("{:#?}", non_empty_lines);
    */

    file_content
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some(url) = parse_url(line) {
                println!("{}", url);
            }
        });

    Ok(())
}