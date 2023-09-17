package main

import (
	"encoding/json"

	maelstrom "github.com/jepsen-io/maelstrom/demo/go"
)

type broadcastStore struct{
	seenMessages []any
}

func newBroadcastStore() *broadcastStore{
	return &broadcastStore{
		seenMessages: make([]any, 0),
	}
}

func handleBroadcastRead(node *maelstrom.Node,bs *broadcastStore) maelstrom.HandlerFunc{
	return func(msg maelstrom.Message) error{ 
		var body map[string]any
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		body["type"] = "read_ok"
		body["messages"] = bs.seenMessages
		return node.Reply(msg,body)
	}
} 

func handleBroadcast(node *maelstrom.Node,bs *broadcastStore, isReplicated bool) maelstrom.HandlerFunc{

	return func(msg maelstrom.Message) error{ 
		var body map[string]any
		
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		
		bs.seenMessages = append(bs.seenMessages, body["message"])
		if(!isReplicated){
			body["type"]="replicated_broadcast"
			for _,n:= range node.NodeIDs(){
				if n!=node.ID(){
					node.Send(n,body)
				}
			}
		} 
		delete(body,"message") 
		body["type"] = "broadcast_ok"
		return node.Reply(msg,body)
	}
}

func handleBroadcastTopology(node *maelstrom.Node,bs *broadcastStore) maelstrom.HandlerFunc{
	return func(msg maelstrom.Message) error{ 
		var body map[string]any
		var response map[string]any = make(map[string]any)
		if err:= json.Unmarshal(msg.Body, &body); err!=nil{
			return err
		}
		response["type"] = "topology_ok"
		return node.Reply(msg,response)
	}
}