fn pascal(rows: usize) -> Vec<Vec<usize>> { 
    (0..rows).map(|r| {
        (1..=rows-r).map(|n| {
            (n..=n+r-1).product::<usize>() / (1..=r).product::<usize>()
        }).collect()}).collect()}

fn main() {
    for i in pascal(20) {
        for j in i { 
            print!("{}", j); 
            for _ in 0..8-j.to_string().len() { print!(" ") } 
        }
        println!();
    }
}
