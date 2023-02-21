
pub mod spot_finder;
pub mod location;

fn main() {
    let nc_res = nats::connect("localhost");
    
    let res = match nc_res {
        Ok(nc) => nc.publish("foo", "my message"), // nc.subscribe("foo"),
        Err(e) => Err(e),
    };
    
    match res {
        Ok(sub) => (), // sub.iter().for_each(|msg| println!("{msg:?}")),
        Err(e) => println!("{e:?}")
    }
}
