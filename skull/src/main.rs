use skol::{Dir, Tape};

fn main() -> anyhow::Result<()> {
    let l = std::io::stdin().lines().next().unwrap()?;
    let mut v = vec![];
    let mut d = None;
    let mut i = None;
    for j in l.split_whitespace() {
        match j {
            ">" => {
                d = Some(Dir::Right);
                i = Some(v.len())
            }
            "<" => {
                d = Some(Dir::Left);
                i = Some(v.len() - 1)
            }
            _ => {
                v.push(j.parse()?);
            }
        }
    }
    let mut v = Tape {
        elements: v,
        idx: i.unwrap(),
        dir: d.unwrap(),
    };
    loop {
        print!("{v} =>");
        let b = v.go();
        if b {
            println!(" halt");
            break;
        } else {
            println!("{v}")
        }
    }
    return Ok(());
}
