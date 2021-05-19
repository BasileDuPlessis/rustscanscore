

type M2x1 = (
    (f32,),
    (f32,)
);
type M2x2 = (
    (f32, f32),
    (f32, f32)
);

mod kalman;
mod simulator;


fn main() {
      
    match simulator::line() {
        Err(e) => println!("{:?}", e),
        _ => ()
    };

}
