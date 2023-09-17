package main

import (
	"encoding/json"
	"log"

	"github.com/google/uuid"
	maelstrom "github.com/jepsen-io/maelstrom/demo/go"
)

func main(){
	n := maelstrom.NewNode()
	
	bs := newBroadcastStore()
	n.Handle("init",func (msg maelstrom.Message) error{
		var body maelstrom.InitMessageBody
		var response map[string]any = make(map[string]any)
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		n.Init(body.NodeID,body.NodeIDs)
		response["type"] = "init_ok"
		return n.Reply(msg,response)
	})
	n.Handle("echo",func (msg maelstrom.Message) error{
		var body map[string]any
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		body["type"] = "echo_ok"
		return n.Reply(msg,body)
	})
	n.Handle("generate",handleUUIDGen(n))
	n.Handle("replicated_broadcast",handleBroadcast(n,bs,true))
	n.Handle("read",handleBroadcastRead(n,bs))
	n.Handle("broadcast",handleBroadcast(n,bs,false))
	n.Handle("topology",handleBroadcastTopology(n,bs))
	if err := n.Run(); err != nil {
		log.Fatal(err)
	}
	
}

func handleUUIDGen(node *maelstrom.Node) maelstrom.HandlerFunc{
	return func(msg maelstrom.Message) error{
		var body map[string]any
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		body["type"]="generate_ok"
		body["id"] = uuid.New()
		return node.Reply(msg,body)
	}
}


