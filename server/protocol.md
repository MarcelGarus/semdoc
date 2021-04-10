## Protocol

This document documents the protocol between the engine process and the viewer.

The protocol uses JSON.

-> { "type": "open", "path": "helloworld.sd" }
<- { "type": "opened", "path": "helloworld.sd" }
-> { "type": "view", "path" }
<- { "type": "" }
