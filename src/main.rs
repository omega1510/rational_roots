mod roots;

fn main() {
    let polynomial: Vec<i64> = vec![1, -2, -3, 4];
    let roots = roots::find_roots(&polynomial);
    println!("{:?}", &roots);
}
