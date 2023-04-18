use std::{
    collections::HashMap, 
    io::Write,
    thread,
    sync::{mpsc::channel,Arc,Mutex}
};

use serde_json::{Value, Map};

use super::protocol::{Message,Body, MessageType, FromMap};

type HandlerFunc<T> = fn (Message) ->Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct MaelstromNode{
    id: Option<String>,
    node_ids: Option<Vec<String>>,
    handlers: HashMap<String, HandlerFunc<Message>>,
    next_message_id: u64,
    callback_handlers: HashMap<String, HandlerFunc<Message>>
}

pub struct Init{
    node_id:String,
    node_ids:Vec<String>
}

impl FromMap for Init{
    fn from_map(map:&Map<String,Value>)->Option<Init>{
        let node_id = Self::get_str_value(map, "node_id".to_string());
        let node_ids = Self::get_str_vec_value(map, "node_ids".to_string());
        match (node_id,node_ids){
            (Some(node_id),Some(node_ids))=>{
                Some(Init{
                    node_id,
                    node_ids
                })
            },
            _ => None
        }
    }
}

impl MaelstromNode{
    pub fn new()->MaelstromNode{ 
        MaelstromNode { 
            id: None, 
            node_ids: None, 
            handlers: HashMap::new(),
            next_message_id: 0,
            callback_handlers: HashMap::new(),
        } 
    }

    pub fn init_node(&mut self,init_payload:Init){
        self.id = Some(init_payload.node_id);
        self.node_ids = Some(init_payload.node_ids); 
    }

    pub fn register_handler(&mut self,message_type:String,func:HandlerFunc<Message>){
        self.handlers.insert(message_type,func);
    }

    pub fn process(
        &mut self,
        msg:Message,
    )->Result<Message,Box<dyn std::error::Error>>{
        match msg.body.message_type{
            MessageType::Init =>{

                if let Ok(mut response) = self.handle_init(msg.to_owned()){
                    self.send(response.dest, response.body)
                } else {
                    Ok(msg)
                }
            }
            _=>{
                match self.handlers.get(&msg.body.message_type.to_string()) {
                    Some(handler_func) =>{
                        if let Ok(mut response) = handler_func(msg.to_owned()){
                            self.reply(&mut response)
                        } else {
                            Ok(msg)
                        }
                    }
                    None => {
                        Ok(msg)
                    }
                }
            }
        }
        
    }


    pub fn handle_init(
        &mut self,
        msg:Message,
    )->Result<Message,Box<dyn std::error::Error>>{

        let init_payload = Init::from_map(&msg.body.payload);
        match init_payload{
            Some(payload)=>{
                self.init_node(payload);
                let response = Message{
                    src: self.id.clone().unwrap_or("a".into()),
                    dest: msg.src.clone(),
                    body:Body{
                        message_type: MessageType::InitOk,
                        in_reply_to: msg.body.msg_id,
                        msg_id: None,
                        ..Default::default()
                    }
                };

                // Delegate to application init handler, if provided
                if let Some(custom_init_handler) = self.handlers.get(&"init".to_string()) {
                    custom_init_handler(msg)
                } else {
                    Ok(response) 
                }
            }
            _=> Ok(msg) //throw error here
        }
        
    }

    fn reply(&mut self,msg:&mut Message)->Result<Message,Box<dyn std::error::Error>>{
        msg.body.set_in_reply_to(msg.body.msg_id.unwrap());
        self.send(msg.src.clone(), msg.body.clone())
    }

    fn send(&self,dest:String,body:Body)->Result<Message,Box<dyn std::error::Error>>{
        let message = Message{
            dest:dest,
            src: self.id.clone().unwrap_or("".into()),
            body:body
        };
        let stdout = &mut std::io::stdout().lock();

        match serde_json::to_writer(&mut *stdout, &message){
            Ok(resp)=>{

                stdout.write_all(b"\n");
            },
            Err(err)=>{}
        }
        Ok(message)

    }

    fn rpc(){

    }
    
    fn sync_rpc(){}

}

