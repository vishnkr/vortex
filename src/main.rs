use std::{collections::HashMap, io::{StdoutLock, StdinLock, Write, Stdout}};

use serde::{Serialize,Deserialize};
use serde_json::{Deserializer};

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Message{
    src: String,
    dest: String,
    body: Body
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Body{
    msg_id: Option<usize>,
    in_reply_to : Option<usize>,

    #[serde(flatten)]
    payload: MessagePayload
}

#[derive(Debug,Clone,Serialize,Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all="snake_case" , tag = "type")]
enum MessagePayload{
    Init { 
        init: String,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk { init_ok:String },

    Echo { echo: String},
    EchoOk{ echo_ok: String}
}

impl MessagePayload {
    fn get_type(&self) -> String {
        match self {
            Self::Init { .. } => "init".to_owned(),
            Self::InitOk { .. } => "init_ok".to_owned(),
            Self::Echo { .. } => "echo".to_owned(),
            Self::EchoOk { .. } => "echo_ok".to_owned(),
        }
    }
}

type HandlerFunc = fn (Message) ->Result<(), Box<dyn std::error::Error>>;


#[derive(Debug)]
struct MaelstromNode{
    id: Option<String>,
    node_ids: Option<Vec<String>>,
    handlers: HashMap<String, HandlerFunc>,
}

#[derive(Debug)]
enum ErrorType{
    HandlerNotRegistered(MessagePayload)
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ErrorType::HandlerNotRegistered(msg_type)=>{
                write!(f,"No handler found for message type {:?}",msg_type)
            }
        }
    }
}
impl std::error::Error for ErrorType {}


impl MaelstromNode{
    fn new()->MaelstromNode{ 
        MaelstromNode { 
            id: None, 
            node_ids: None, 
            handlers: HashMap::new(),
        } 
    }

    fn init_node(&mut self,id:String, node_ids:Vec<String>){
        self.id = Some(id);
        self.node_ids = Some(node_ids); 
    }

    fn register_handler(&mut self,payload:MessagePayload,func:HandlerFunc){
        self.handlers.insert(payload.get_type(),func);
    }

    fn handle_message(
        &mut self,
        msg:Message,
        stdout: &mut StdoutLock
    )->Result<(),Box<dyn std::error::Error>>{

        match self.handlers.get(&msg.body.payload.get_type()) {
            Some(handler_func) =>{
                handler_func(msg.to_owned())
            }
            None => Err(ErrorType::HandlerNotRegistered(msg.body.payload).into())
        }
    }

    fn handle_init(
        &mut self,
        msg:Message,
    )->Result<(),Box<dyn std::error::Error>>{

        let msg_clone = msg.clone();
        match msg.body.payload {
            MessagePayload::Init { node_id, node_ids, .. } => {
                self.init_node(node_id, node_ids);

                // Delegate to application init handler, if provided
                if let Some(custom_init_handler) = self.handlers.get(&"init".to_string()) {
                    custom_init_handler(msg_clone)
                } else {
                    Ok(())
                }
            },
            _ =>  Ok(()),
        }
    }

    fn reply(&self,msg:Message, body:Body){

    }

    fn send(&self,body:Body, dest:String){
        let response = Message{
            src: self.id.unwrap_or("default".to_string()),
            dest,
            body
        };

        //serde_json::to_writer(*self.stdout, &response);
        self.stdout.write_all(b"\n");
    }
}

fn echo(msg:Message)->Result<(),Box<dyn std::error::Error>>{
    msg.body.payload = MessagePayload::EchoOk { echo_ok }
    Ok(())
}

fn main() {
    let stdin = std::io::stdin().lock();
    let deserializer = Deserializer::from_reader(stdin).into_iter::<Message>();
    
    let stdout = std::io::stdout().lock();
    let node = &mut MaelstromNode::new();

    for line in deserializer {
        match line {
            Ok(msg) => {
                if  let MessagePayload::Init{..} = msg.body.payload {
                    if let Err(err) = node.handle_init(msg.to_owned(),&mut stdout){
                        print!("Node Initialization Error - {}",err)
                    }
                }
                else if let Err(err) = node.handle_message(msg,&mut stdout){
                    println!("Error handling message - {}",err);
                }
            }
            Err(err) => {
                println!("Error deserializing message - {}", err);
            }
        }
    }
}

