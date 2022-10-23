use bincode::{config, Decode, Encode};
use borsh::{BorshDeserialize, BorshSerialize};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::time::Instant;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::{accept_hdr, Message, WebSocket};

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
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            let msg = websocket.read_message().unwrap();
            let mgs = match msg.clone() {
                Message::Text(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Binary(value) => value,
                Message::Ping(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Pong(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Close(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Frame(_) => {
                    let vec = vec![];
                    vec
                }
            };
            let config = config::standard();
            let (world, _): (Vec<World>, usize) =
                bincode::decode_from_slice(&mgs[..], config).unwrap();
            let msg = websocket.read_message().unwrap();
            let mgs = match msg.clone() {
                Message::Text(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Binary(value) => value,
                Message::Ping(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Pong(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Close(_) => {
                    let vec = vec![];
                    vec
                }
                Message::Frame(_) => {
                    let vec = vec![];
                    vec
                }
            };
            let (world_borsh, _): (Vec<WorldBorsh>, usize) =
                bincode::decode_from_slice(&mgs[..], config).unwrap();
            encode_all_data(world, world_borsh, websocket);
        });
    }
}

pub fn encode_all_data(
    world: Vec<World>,
    world_borsh: Vec<WorldBorsh>,
    mut websocket: WebSocket<TcpStream>,
) {
    let config = config::standard();
    println!("Start encode for a data \n");
    let mut vec = vec![];
    let now_bincode = Instant::now();
    for world in &world {
        let encoded: Vec<u8> = bincode::encode_to_vec(world, config).unwrap();
        vec.push(encoded);
    }
    let after_loop_bincode = Instant::now();
    println!(
        "Time for encode to bincode: {:?}\n this len value: \n {:?} \n",
        after_loop_bincode - now_bincode,
        vec.len()
    );
    websocket
        .write_message(Message::Text("End test for bincode!".to_string()))
        .unwrap();

    let mut vec_borsh = vec![];
    let now_borsh = Instant::now();
    for world in &world_borsh {
        let encoded = world.try_to_vec().unwrap();
        vec_borsh.push(encoded);
    }
    let after_loop_borsh = Instant::now();
    println!(
        "Time for encode to borsh: {:?}\n with this len value: \n {:?} \n",
        after_loop_borsh - now_borsh,
        vec.len()
    );
    websocket
        .write_message(Message::Text("End test for borsh!".to_string()))
        .unwrap();

    let mut vec_json = vec![];
    let now_json = Instant::now();
    for world in &world {
        let encoded = json!({ "Value": world });
        vec_json.push(encoded);
    }
    let after_loop_json = Instant::now();
    println!(
        "Time for encode to json: {:?}\n this len value: \n {:?} \n",
        after_loop_json - now_json,
        vec.len()
    );
    websocket
        .write_message(Message::Text("End test for json!".to_string()))
        .unwrap();
}

pub fn decode_all_data(
    world: Vec<World>,
    world_borsh: Vec<WorldBorsh>,
    mut websocket: WebSocket<TcpStream>,
) {
    let config = config::standard();
    println!("Start decode for a data \n");
    let mut vec_bincode_encode = vec![];
    let mut vec = vec![];
    for world in &world {
        let encoded: Vec<u8> = bincode::encode_to_vec(world, config).unwrap();
        vec.push(encoded);
    }
    let now_bincode = Instant::now();
    for world in vec {
        let config = config::standard();
        let (decoded, _len): (World, usize) =
            bincode::decode_from_slice(&world[..], config).unwrap();
        vec_bincode_encode.push(decoded);
    }
    let after_loop_bincode = Instant::now();
    println!(
        "Time for decode to bincode: {:?} \n this len value: \n {:?} \n",
        after_loop_bincode - now_bincode,
        vec_bincode_encode.len()
    );
    websocket
        .write_message(Message::Text("End test for json!".to_string()))
        .unwrap();

    let mut vec_borsh_encode = vec![];
    let mut vec = vec![];
    for world in &world_borsh {
        let encoded: Vec<u8> = world.try_to_vec().unwrap();
        vec.push(encoded);
    }
    let now_borsh = Instant::now();
    for world in vec {
        let decoded: WorldBorsh = WorldBorsh::try_from_slice(&world).unwrap();
        vec_borsh_encode.push(decoded);
    }
    let after_loop_borsh = Instant::now();
    println!(
        "Time for decode to borsh: {:?} \n this len value: \n {:?} \n",
        after_loop_borsh - now_borsh,
        vec_borsh_encode.len()
    );
    websocket
        .write_message(Message::Text("End test for json!".to_string()))
        .unwrap();

    let mut vec_json_encode = vec![];
    let mut vec = vec![];
    for world in &world {
        let encoded = serde_json::to_string(world).unwrap();
        vec.push(encoded);
    }
    let now_json = Instant::now();
    for world in vec {
        let decoded: World = serde_json::from_str(&*world).unwrap();
        vec_json_encode.push(decoded);
    }
    let after_loop_json = Instant::now();
    println!(
        "Time for decode to json: {:?} \n this len value: \n {:?} \n",
        after_loop_json - now_json,
        vec_json_encode.len()
    );
    websocket
        .write_message(Message::Text("End test for json!".to_string()))
        .unwrap();
}
