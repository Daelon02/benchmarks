use bincode::{config, Decode, Encode};
use borsh::{BorshDeserialize, BorshSerialize};
use std::time::Instant;
use rand::Rng;
use serde_json::{json};
use serde::{Serialize, Deserialize};

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

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
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

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
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

    let world = World::many_rand(0, 1000000);

    let world_borsh = WorldBorsh::many_rand(0, 1000000);

    encode_all_data(world.clone(), world_borsh.clone());
    decode_all_data(world, world_borsh);
}

pub fn encode_all_data(world: Vec<World>, world_borsh: Vec<WorldBorsh>) {
    let config = config::standard();
    println!("Start encode for a data \n");
    let mut vec = vec![];
    let now_bincode = Instant::now();
    for world in &world {
        let encoded: Vec<u8> = bincode::encode_to_vec(world, config).unwrap();
        vec.push(encoded);
    }
    let after_loop_bincode = Instant::now();
    println!("Time for encode to bincode: {:?}\n this len value: \n {:?} \n", after_loop_bincode - now_bincode, vec.len());

    let mut vec_borsh = vec![];
    let now_borsh = Instant::now();
    for world in &world_borsh {
        let encoded = world.try_to_vec().unwrap();
        vec_borsh.push(encoded);
    }
    let after_loop_borsh = Instant::now();
    println!("Time for encode to borsh: {:?}\n with this len value: \n {:?} \n", after_loop_borsh - now_borsh, vec.len());

    let mut vec_json = vec![];
    let now_json = Instant::now();
    for world in &world {
        let encoded = json!({"Value": world});
        vec_json.push(encoded);
    }
    let after_loop_json = Instant::now();
    println!("Time for encode to json: {:?}\n this len value: \n {:?} \n", after_loop_json - now_json, vec.len());
}

pub fn decode_all_data(world: Vec<World>, world_borsh: Vec<WorldBorsh>) {
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
        let (decoded, _len): (World, usize) = bincode::decode_from_slice(&world[..], config).unwrap();
        vec_bincode_encode.push(decoded);
    }
    let after_loop_bincode = Instant::now();
    println!("Time for decode to bincode: {:?} \n this len value: \n {:?} \n", after_loop_bincode - now_bincode, vec_bincode_encode.len());

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
    println!("Time for decode to borsh: {:?} \n this len value: \n {:?} \n", after_loop_borsh - now_borsh, vec_borsh_encode.len());

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
    println!("Time for decode to json: {:?} \n this len value: \n {:?} \n", after_loop_json - now_json, vec_json_encode.len());
}