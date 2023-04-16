
mod maelstrom;
use std::io::BufRead;


use crate::maelstrom::{
    node::MaelstromNode,
    protocol::{Message}
};


fn echo_handler(msg: Message)->Result<Message,Box<dyn std::error::Error>>{
    let mut response = msg.clone();
    response.body.set_type("echo_ok");
    Ok(response)
}

fn main(){
    let stdin = std::io::stdin().lock();
    let mut node = MaelstromNode::new();
    node.register_handler("echo".to_string(), echo_handler);

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
