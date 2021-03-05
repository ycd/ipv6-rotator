use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpBlock {
    CIDR32,
    CIDR48,
    CIDR64,
}

#[derive(Debug, PartialEq)]
pub struct Rotator<'a> {
    pub device: &'a str,
    pub sleep_time: u16,
    pub block: IpBlock,
    pub network: &'a str,
    pub count: u16,
    addresses: Box<Vec<String>>,
    available_chars: Vec<String>,
}

impl<'a> Rotator<'a> {
    pub fn builder() -> Self {
        Self {
            device: "",
            sleep_time: 10,
            block: IpBlock::CIDR64,
            network: "",
            count: 5,
            addresses: Box::new(Vec::new()),
            available_chars: vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c", "d", "e", "f",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    // Set the ip block
    pub fn block(&mut self, block: IpBlock) -> &mut Self {
        self.block = block;
        self
    }

    pub fn network(&mut self, network: &'a str) -> &mut Self {
        self.network = network;
        self
    }

    pub fn sleep_time(&mut self, sleep_time: u16) -> &mut Self {
        self.sleep_time = sleep_time;
        self
    }

    pub fn device(&mut self, device: &'a str) -> &mut Self {
        self.device = device;
        self
    }

    pub fn count(&mut self, count: u16) -> &mut Self {
        self.count = count;
        self
    }

    pub fn build(&mut self) -> Self {
        Self {
            block: self.block,
            count: self.count,
            device: self.device,
            network: self.network,
            sleep_time: self.sleep_time,
            addresses: Box::new(self.addresses.to_vec()),
            available_chars: vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c", "d", "e", "f",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }
}

impl Rotator<'_> {
    pub fn rotate(&mut self) {}

    pub fn create_unique_ip(&mut self) -> String {
        match self.block {
            IpBlock::CIDR32 => format!(
                "{}:{}:{}:{}:{}:{}:{}",
                &self.network,
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen()
            ),
            IpBlock::CIDR48 => format!(
                "{}:{}:{}:{}:{}:{}",
                &self.network,
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen()
            ),
            IpBlock::CIDR64 => format!(
                "{}:{}:{}:{}:{}",
                &self.network,
                self.gen(),
                self.gen(),
                self.gen(),
                self.gen()
            ),
        }
    }

    fn gen(&self) -> String {
        vec![
            self.available_chars
                .choose(&mut rand::thread_rng())
                .unwrap(),
            self.available_chars
                .choose(&mut rand::thread_rng())
                .unwrap(),
            self.available_chars
                .choose(&mut rand::thread_rng())
                .unwrap(),
            self.available_chars
                .choose(&mut rand::thread_rng())
                .unwrap(),
        ]
        .iter()
        .flat_map(|s| s.chars())
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_builder_success() {
        let rotator = self::Rotator::builder()
            .device("eth0")
            .network("2001:db8:0000:0000")
            .count(5)
            .block(IpBlock::CIDR48)
            .sleep_time(15)
            .build();

        assert_eq!(
            self::Rotator {
                device: "eth0",
                network: "2001:db8:0000:0000",
                count: 5,
                block: IpBlock::CIDR48,
                sleep_time: 15,
                addresses: Box::new(Vec::new()),
                available_chars: vec![
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c", "d", "e", "f",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            },
            rotator
        );
    }

    #[test]
    fn test_unique_ip() {}
}
