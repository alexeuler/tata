mod command_line;

#[async_std::main]
async fn main() {
    command_line::start_command_line().await
}
