mod args;

fn main() {
    let matches = args::get_args();
    println!("{:?}", matches);
}
