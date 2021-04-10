
type CommandId = u64;

struct Command {
    id: CommandId,
    action: Action,
}

enum CommandAction {
    Resource(ResourceAction),
    Load(BlockId),
}

enum ResourceAction {
    OpenFile { path: String },
    OpenStream,
    AppendToStream(ResId, Vec<u8>),
    CloseResource(ResId),
}

enum Response {
    Resource(ResourceEvent),
    Load(LoadAction),
}

enum ResourceEvent {
    Opened(ResId),
    FileNotFound,
}

enum LoadResponse {
    Error,
    Loaded,
}
