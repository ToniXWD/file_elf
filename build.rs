fn main() {
    println!("cargo:rustc-link-search=native=C:/Users/toni/miniconda3/Library/lib");
    println!("cargo:rustc-link-lib=static=sqlite3");
}