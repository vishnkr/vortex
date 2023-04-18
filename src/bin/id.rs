use vortex::maelstrom::{protocol::*, node::MaelstromNode, main_loop};

fn generate_unique_id(msg: Message)->Result<Message,Box<dyn std::error::Error>>{
    Ok(msg)
}

fn main(){
    let mut node = MaelstromNode::new();
    node.register_handler(MessageType::Echo.to_string(), generate_unique_id);
    main_loop(node)
}