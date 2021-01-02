use libp2p::{
    floodsub::{Topic}
};



pub struct Peer {
    topics: Vec<Topic>
}


impl Peer {
    /// create a new peer 
    pub fn new() -> Self {
        Self {
            topics: vec![]
        }
    }
    
    /// Create a new topic and subscribe it to the network
    pub fn join_topic(&mut self, topic_name: String) {
        let topic = Topic::new(topic_name);

        // subscribe it to the network 
        if true /*Insert subscription code*/{
            self.topics.push(topic);
        }
    }

    /// Get a list of all topics subscribed 
    pub fn get_topics(&self) -> Vec<String> {
        self.topics.iter()
            .map(|topic| topic.id().to_string())
            .collect()
    }
}