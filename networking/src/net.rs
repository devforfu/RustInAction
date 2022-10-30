use std::str::FromStr;

use libnet::PageReaderProtocol;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().nth(1);

    let name = arg.expect("usage: net <protocol>");

    if let Ok(protocol) = PageReaderProtocol::from_str(name.as_ref()) {
        let reader = protocol.create_reader();
        let url = "www.rustinaction.com";
        let content = reader.read_page(url)?;
        println!("{}", content);
    }

    Ok(())
}
