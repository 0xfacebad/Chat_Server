use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt,BufReader};
mod rc_four;
 async fn handle_connections(socket:TcpStream)->Result<(), std::io::Error>{
    let mut reader = BufReader::new(socket);
    loop {

            let mut buffer = String::new();
            let read_bytes = reader.read_line(&mut buffer).await?;
            if read_bytes == 0 {return Ok(())}
            let res = rc_four::RcFour::new(buffer);
            let decrpy_res = res.decrypt();
             match String::from_utf8(decrpy_res){
                   Ok(string)=>println!("{} " , string),
                   Err(err)=>eprintln!("Error {}" ,err),
             }
            // let send_val = res.encryption();
            // let stream = reader.get_mut();
            // stream.write_all(send_val.as_slice()).await?;

        //     let dec = res.decrypt();
        //     match String::from_utf8(dec) {
        // Ok(string) => println!("Converted string: {}", string),
        // Err(e) => eprintln!("Error converting Vec<u8> to String: {}", e),
    }
    


  Ok(())
}
#[tokio::main]
async fn main()->Result<(),std::io::Error>{

       let listener = match TcpListener::bind("127.0.0.1:8000").await{
                        Ok(listener)=>listener,
                        Err(err)=>{eprintln!("Failed error was {}",err);
                        return Err(err);
                    }
       };
       loop{
             let (socket ,ip) = listener.accept().await.unwrap();
             println!("Got Connection from {} :port {} ", ip.ip() , ip.port());

             tokio::spawn(async move {
                 if let Err(err) = handle_connections(socket).await{
                        eprintln!("Error handling connection: {}", err);
                 }
             });
       }



      Ok(())
}