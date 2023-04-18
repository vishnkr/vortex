use core::fmt;
use std::{str::FromStr};

use serde::{Serialize,Deserialize};
use serde_json::{Map,Value};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Message{
    pub src: String,
    pub dest: String,
    pub body: Body
}

#[derive(Debug,Clone,Serialize,Deserialize,Eq, PartialEq, Default)]
pub struct Body{
    #[serde(rename="type", default)]
    pub message_type : MessageType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to : Option<u64>,

    #[serde(flatten)]
    pub payload: Map<String,Value>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct InitMessageBody {
    #[serde(default)]
    pub node_id: String,

    #[serde(rename = "node_ids", default)]
    pub nodes: Vec<String>,
}

#[derive(Debug,Clone,Serialize,Deserialize,Eq, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum MessageType{
    #[default]
    Init,
    InitOk,

    Echo,
    EchoOk,

    Generate,
    GenerateOk
}

pub trait FromMap{
    fn get_str_value(map:&Map<String,Value>,key:String)->Option<String>{
        Some(map.get(&key).unwrap().as_str().unwrap().to_string())
    }

    fn get_str_vec_value(map:&Map<String,Value>,key:String)->Option<Vec<String>>{
        Some(map.get(&key).unwrap().as_array().unwrap()
        .iter()
        .map(|n| n.as_str().unwrap().to_string())
        .collect::<Vec<String>>())
    }
    
    fn from_map(map:&Map<String,Value>)->Option<Self> where Self: Sized;
}

impl FromStr for MessageType{
    type Err = ();

    fn from_str(input: &str) -> Result<MessageType, ()> {
        match input {
            "init"  => Ok(MessageType::Init),
            "init_ok"  => Ok(MessageType::InitOk),
            "echo"  => Ok(MessageType::Echo),
            "echo_ok" => Ok(MessageType::EchoOk),
            "generate" => Ok(MessageType::Generate),
            "generate_ok" => Ok(MessageType::GenerateOk),
            _      => Err(()),
        }
    }
}

impl fmt::Display for MessageType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::Init => write!(f,"init"),
            MessageType::InitOk => write!(f,"init_ok"),
            MessageType::Echo => write!(f,"echo"),
            MessageType::EchoOk => write!(f,"echo_ok"),
            MessageType::Generate => write!(f,"generate"),
            MessageType::GenerateOk => write!(f,"generate_ok"),
        }
    }
}

impl Message{
    pub fn get_type(&self)->String{
        return self.body.message_type.to_string();
    }
}

impl Body{
    pub fn new()->Self{ Self::default()}

    pub fn from_payload(payload:Map<String,Value>) ->Self{
        Body{
            payload,
            ..Default::default()
        }
    }

    pub fn set_type(&mut self,typ: &str){
        self.message_type = MessageType::from_str(typ).unwrap();
    }

    pub fn set_in_reply_to(&mut self, reply_to : u64) {
        self.in_reply_to = Some(reply_to);
    }

}
