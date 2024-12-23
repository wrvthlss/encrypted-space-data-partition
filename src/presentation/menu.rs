use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub async fn display_menu(writer: &mut tokio::io::WriteHalf<TcpStream>) -> Result<(), std::io::Error> {
    let menu = "\
        Please select a following option:\r\n\
        1. Enter existing username.\r\n\
        2. Enter new username to register.\r\n\
        3. Select this option to be assigned a read-only guest account and be assigned a random username.\r\n\
        4. Disconnect from the server.\r\n\r\n\
        Please enter 1, 2, 3 or 4:\r\n";

    writer.write_all(menu.as_bytes()).await?;
    Ok(())
}
