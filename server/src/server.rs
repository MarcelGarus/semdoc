type ResId = u64;
type BlockId = u64;

struct Server {
    resources: Vec<Resource>,
}
impl Server {
    pub fn openFile(path: String): Result<ResId> {}
    pub fn openStream(): ResId {}
    pub fn close(resource: ResId) {}
    pub fn load(id: BlockId): Result<Block> {}
}

trait Resource {
    pub fn get_byte_at(position: usize) -> u8
}

// TODO: Make this memory-mapped.
struct File {
    file: std::io::File,
}

struct Stream {
    bytes: Vec<u8>,
}
impl Stream {
    fn add(&mut self, bytes: &[u8]) {
        bytes.push(bytes);
    }
}
