mod kalman;
mod simulator;


fn main() {
      
    match simulator::line() {
        Err(e) => println!("{:?}", e),
        _ => ()
    };

}
