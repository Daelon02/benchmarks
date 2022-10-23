use std::time::Instant;
use bincode::{config, Decode, Encode};
use borsh::{BorshDeserialize, BorshSerialize};
use rand::Rng;
use serde::{Serialize, Deserialize};
use tungstenite::{connect, Message};
use url::Url;

#[derive(Encode, Decode, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct EntityBincode {
    pub string: String,
    pub number: u64,
    pub number_float: f64,
    pub vec: Vec<u32>,
    pub bool: bool,
    pub string1: String,
    pub number1: u64,
    pub number_float1: f64,
    pub vec1: Vec<u32>,
    pub bool1: bool,
}

#[derive(Encode, Decode, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct World(pub Vec<EntityBincode>);

impl World {
    pub fn many_rand(from: u64, to: u64) -> Vec<World> {
        let mut rng = rand::thread_rng();
        let mut vec = vec![];
        for _i in from..to {
            let world = World(vec![
                EntityBincode {
                    string: "TestString".to_string(),
                    number: rng.gen(),
                    number_float: rng.gen(),
                    vec: vec![rng.gen()],
                    bool: true,
                    string1: "TestString".to_string(),
                    number1: rng.gen(),
                    number_float1: rng.gen(),
                    vec1: vec![rng.gen()],
                    bool1: true,
                },
                EntityBincode {
                    string: "TestAnotherString".to_string(),
                    number: rng.gen(),
                    number_float: rng.gen(),
                    vec: vec![rng.gen(), rng.gen(), rng.gen()],
                    bool: false,
                    string1: "TestString".to_string(),
                    number1: rng.gen(),
                    number_float1: rng.gen(),
                    vec1: vec![rng.gen()],
                    bool1: true,
                },
            ]);
            vec.push(world);
        }
        vec
    }
}

#[derive(Encode, Decode, BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub struct EntityBorsh {
    pub string: String,
    pub number: u64,
    pub number_float: f64,
    pub vec: Vec<u32>,
    pub bool: bool,
    pub string1: String,
    pub number1: u64,
    pub number_float1: f64,
    pub vec1: Vec<u32>,
    pub bool1: bool,
}

#[derive(Encode, Decode, BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub struct WorldBorsh(pub Vec<EntityBorsh>);

impl WorldBorsh {
    pub fn many_rand(from: u64, to: u64) -> Vec<WorldBorsh> {
        let mut rng = rand::thread_rng();
        let mut vec = vec![];
        for _i in from..to {
            let world = WorldBorsh(vec![
                EntityBorsh {
                    string: "TestString".to_string(),
                    number: rng.gen(),
                    number_float: rng.gen(),
                    vec: vec![rng.gen()],
                    bool: true,
                    string1: "TestString".to_string(),
                    number1: rng.gen(),
                    number_float1: rng.gen(),
                    vec1: vec![rng.gen()],
                    bool1: true,
                },
                EntityBorsh {
                    string: "TestAnotherString".to_string(),
                    number: rng.gen(),
                    number_float: rng.gen(),
                    vec: vec![rng.gen(), rng.gen(), rng.gen()],
                    bool: false,
                    string1: "TestString".to_string(),
                    number1: rng.gen(),
                    number_float1: rng.gen(),
                    vec1: vec![rng.gen()],
                    bool1: true,
                },
            ]);
            vec.push(world);
        }
        vec
    }
}

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let world = World::many_rand(0, 100000);

    let world_borsh = WorldBorsh::many_rand(0, 100000);

    let config = config::standard();
    let encoded_world: Vec<u8> = bincode::encode_to_vec(world, config).unwrap();

    let encoded_world_borsh: Vec<u8> = bincode::encode_to_vec(world_borsh, config).unwrap();

    let now = Instant::now();
    socket.write_message(Message::Binary(encoded_world)).unwrap();
    socket.write_message(Message::Binary(encoded_world_borsh)).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let after_loop = Instant::now();
        println!("Received: {}, from {:?}", msg, after_loop - now);
    }
    // socket.close(None);

    // encode_all_data(world.clone(), world_borsh.clone());
    // decode_all_data(world, world_borsh);
}
