#[macro_use]
extern crate redis_module;

use coredump::register_panic_handler;
use nix::unistd::{fork, ForkResult};
use redis_module::{Context, RedisError, RedisResult, REDIS_OK};

fn fork_child(_ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 1 {
        return Err(RedisError::WrongArity);
    }

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("New child pid: {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("I'm a new child process");
        }
        Err(_) => println!("Fork failed"),
    }

    REDIS_OK
}

fn dump_child(_ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 1 {
        return Err(RedisError::WrongArity);
    }

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("New child pid: {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("I'm a new child process");
            register_panic_handler().unwrap();
            panic!("induced panic");
        }
        Err(_) => println!("Fork failed"),
    }

    REDIS_OK
}

redis_module! {
    name: "redisdebug",
    version: 001000,
    data_types: [],
    commands: [
        ["debug.dump", dump_child, ""],
        ["debug.fork", fork_child, ""],
    ]
}
