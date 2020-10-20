use protocol::wire::middleware::Pipeline;

#[derive(Debug, Clone)]
pub struct NullPipeline;

impl Pipeline for NullPipeline {
    fn encode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, protocol::Error> {
        println!("{:?}", data);
        Ok(data)
    }

    fn decode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, protocol::Error> {
        println!("{:?}", data);
        Ok(data)
    }
}