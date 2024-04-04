use std::io::{self, Write};
use color_eyre::{eyre::ensure, Result};

const PM: i32 = 1000_0000;
const EX: i32 = 980_0000;
const AA: i32 = 950_0000;

fn prompt(msg: &str) -> Result<String> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(msg.as_bytes())?;
    stdout.flush()?;
    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;
    Ok(ans)
}

fn validate_difficulty(difficulty: &str) -> Result<f32> {
    let result = difficulty.parse::<f32>()?;
    let d = result * 10.0;
    let i = d.trunc();

    ensure!(d == i && d <= 120.0, "Invalid difficulty");

    let is_legal = match d as u16 {
        80..=120 => true,
        70|75|78 => true,
        d if d < 70 => d%10 == 5 || d%10 == 0,
        _ => false
    };

    ensure!(is_legal, "Illegal difficulty");

    Ok(result)
}

fn main() -> Result<()> {
    let difficulty = validate_difficulty(prompt("Song difficulty: ")?.trim_end())?;
    let score = prompt("Score: ")?.trim_end().parse::<i32>()?;
    ensure!(score >= 0, "Invalid score");

    let ptt = if score >= PM {
        difficulty + 2.0
    } else if score >= EX {
        difficulty + 1.0 + ((score - EX) / 200000) as f32
    } else {
        difficulty + ((score - AA) / 300000) as f32
    };

    if ptt.is_sign_negative() {
        println!("0")
    } else {
        println!("{}", ptt)
    }

    Ok(())
}
