use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        println!("In group:");
        let mut sum = 0;

        for line in group.lines() {
            let value = line.parse::<u64>()?;
            println!("  - {value}");
            sum += value;
        }
        println!("Group has sum {sum}")
    } //println!("{input}");
    Ok(())
}
/*fn read_input() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
    Ok(input)
    //fs_err::read_to_string("src/input.tx")

    /*let path = "src/input.tx";
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(PathedIoError {
            path: path.into(),
            inner: e,
        }),
    }*/
}*/
//#[derive(Debug)]
struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathedIoError")
            .field("path", &self.path)
            .field("inner", &self.inner)
            .finish()
    }
}
