use std::io;
use std::io::{BufRead, Write};

fn main() {

    // Reading resources directory input ----------------------------------------------------

    println!("\n Enter the webpage's local path: \n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let os_string: std::ffi::OsString = input.trim().trim_matches('"').into();
    // println!("{:?}", os_string);

    // Reading local port input -------------------------------------------------------------

    println!("\n Enter local port's IPv4 address number: \n");
    let mut port = String::new();
    let _ = io::stdin().read_line(&mut port);
    // println!("User input: {}", port);

    // Defining local hostname to serve the files ------------------------------------------------

    let mut hostname = "127.0.0.1:".to_string();
    hostname.push_str(&port);
    // println!("User input: {}", hostname);

    // Opening browser to request from chosen the hostname and port-------------------------------

    // origin = protocol//:hostname:port/
    let mut origin: String = "".to_string();
    let protocol = "http://";

    origin.push_str(&protocol.to_string());
    origin.push_str(&hostname.to_string());
    origin.push_str(&port.to_string());

    let _ = open::that(&origin);

    // Defining server ---------------------------------------------------------------------

    let listener = std::net::TcpListener::bind(hostname.trim()).unwrap();
    println!("\n Listening started, ready to accept connections. \n");

    
    for mut stream in listener.incoming().flatten(){
        
        let mut reader = std::io::BufReader::new(&mut stream);
        let mut line  = String::new(); 
        let _ = reader.read_line(&mut line).unwrap();
        print!("\n ----------------------------------------------------------------------- \n");
        print!("\n {line}");
        

        match line.trim().split(' ').collect::<Vec<_>>().as_slice() {
            
            // Root route -----------------------------------------------------------------
            ["GET", resource, "HTTP/1.1"] => {

                // Printing request
                loop
                {
                let mut line  = String::new();
                reader.read_line(&mut line).unwrap();
                print!("{line}");
                if line.trim().is_empty() {break; }   
                }

                // Defining request's url path
                let mut path = std::path::PathBuf::from(&os_string); 
                path.push(resource.trim_start_matches("/"));
                if resource.ends_with('/') { path.push("index.html");}
                println!("\n Resource: {}", path.display());

                // Defining Response
                let status_line = "HTTP/1.1 200 OK";
                let mut headers = format!("{status_line}\r\n\r\n");

                let pattern = ".js";
                if path.display().to_string().trim().ends_with(pattern) {
                    let content_type = "Content-Type: Application/javascript; charset=UTF-8";                 
                    headers = format!("{status_line}\r\n{content_type}\r\n\r\n");
                }
                
                // Dispaching response
                stream.write(headers.as_bytes()).unwrap();
                stream.write_all(&std::fs::read(path).unwrap()).unwrap();
                stream.flush().unwrap();

            }
            _ => todo!()
        }

    }
} 
