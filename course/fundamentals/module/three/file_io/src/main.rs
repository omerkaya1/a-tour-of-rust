use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// taken from the book rust by example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn line_count_sync(filename: String) -> anyhow::Result<usize> {
    let now = std::time::Instant::now();
    let mut count = 0;

    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    count += 1
                }
            }
        });
    }

    println!("Read {} lines in {:.3} seconds", count, now.elapsed().as_secs_f32());
    Ok(count)
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;

    println!("reading a file: {filename}");
    let now = std::time::Instant::now();

    let mut count = 0;

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            count += 1;
        }
    }

    println!("Read {} lines in {:.3} seconds", count, now.elapsed().as_secs_f32());
    Ok(count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("reading a large file");
    let now = std::time::Instant::now();
    
    let (c1, c2) = tokio::join!(
        line_count("temp.txt".to_string()),
        line_count("temp.txt".to_string()),
    );

    println!("total lines: {}", c1? + c2?);
    println!("read in {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
