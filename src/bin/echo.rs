use vortex::maelstrom::{protocol::*, node::MaelstromNode, main_loop};
use serde_json::{Value, Map};

pub fn echo_handler(msg: Message)->Result<Message,Box<dyn std::error::Error>>{
    let mut response = msg.clone();
    response.body.set_type("echo_ok");
    Ok(response)
}
pub struct Echo{
    echo:String,
}

impl FromMap for Echo{
    fn from_map(map:&Map<String,Value>)->Option<Echo>{
        let echo = map.get(&MessageType::Echo.to_string());
        match echo{
            Some(value)=>{
                Some(Echo {
                    echo: value.to_string()
                })
            }
            None => None
        }
    }
}

fn main(){
    let mut node = MaelstromNode::new();
    node.register_handler(MessageType::Echo.to_string(), echo_handler);
    main_loop(node)
}

