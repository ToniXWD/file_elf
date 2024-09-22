use file_elf::launch_elf;

#[rocket::main]
async fn main() {
    launch_elf().await;
}
