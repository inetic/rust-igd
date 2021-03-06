extern crate igd;

fn main() {
    match igd::search_gateway() {
        Err(ref err) => match *err {
            igd::SearchError::IoError(ref ioe) => println!("IoError: {}", ioe),
            _ => println!("{:?}", err),
        },
        Ok(gateway) => {
            match gateway.remove_port(igd::PortMappingProtocol::TCP,
                                   80) {
                Err(ref err) => match *err {
                    igd::RequestError::IoError(ref ioe) => {
                        println!("IoError: {}", ioe)
                    },
                    _ => println!("{:?}", err),
                },
                Ok(()) => {
                    println!("It worked");
                },
            }
        },
    }
}
