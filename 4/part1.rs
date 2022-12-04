use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut count: u32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }

        let mut elfs = line.split(",");
        let elf1 = elfs.next().unwrap();
        let elf2 = elfs.next().unwrap();

        let mut elf1_chunks = elf1.split("-");
        let elf1_start: u32 = elf1_chunks
            .next()
            .unwrap()
            .parse()
            .expect("could not parse line to integer");
        let elf1_end: u32 = elf1_chunks
            .next()
            .unwrap()
            .parse()
            .expect("could not parse line to integer");

        let mut elf2_chunks = elf2.split("-");
        let elf2_start: u32 = elf2_chunks
            .next()
            .unwrap()
            .parse()
            .expect("could not parse line to integer");
        let elf2_end: u32 = elf2_chunks
            .next()
            .unwrap()
            .parse()
            .expect("could not parse line to integer");

        if (elf1_start <= elf2_start && elf1_end >= elf2_end)
            || (elf2_start <= elf1_start && elf2_end >= elf1_end)
        {
            count+=1;
        }
    }

    println!("{:?} elfs have nothing to do", count);
    Ok(())
}
