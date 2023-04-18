use self::node::MaelstromNode;
use std::io::BufRead;
use self::protocol::Message;

pub mod node;
pub mod protocol;


pub fn main_loop(mut node:MaelstromNode){
    let stdin = std::io::stdin().lock();
    for line in stdin.lines(){
        match line {
            Ok(msg)=>{
                let input_msg = serde_json::from_str::<Message>(&msg);
                match input_msg{
                    Ok(input_msg) => {
                        let ret_msg = node.process(input_msg);
                        match ret_msg {
                            Ok(message) => {},
                            Err(err)=>{println!("got error {}",err)}
                        } 
                    },
                    Err(err) => {
                        println!("Error deserializing message - {}", err);
                    }
                }
                
            }
            Err(err) => println!("Error reading from STDIN - {}", err)
        }
       

    }
}