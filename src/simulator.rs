use image::{ImageBuffer, Rgb};


pub fn line() -> Result<(), std::io::Error> {

    let mut img = ImageBuffer::from_fn(512, 512, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    

    let measurements = sample_line_gen();
    let mut measurement_iter = measurements.iter();

    let mut last_measure= *measurement_iter.next().unwrap(); 

    let mut dt = 1.0;

    let mut x = (
        (last_measure,),
        (0.0,)
    );

        
    let mut p = (
        (1.0, 0.0),
        (0.0, 1.0)
    );

    let h = (
        (1.0, 0.0),
        (0.0, 1.0)
    );
    let r = (
        (1.0, 0.0),
        (0.0, 1.0)
    );

    let mut vec = Vec::<(u32, u32, u32)>::new();
    vec.push((dt as u32, last_measure as u32, last_measure as u32));


    for measure in measurement_iter {

        let a = (
            (1.0, 1.0),
            (0.0, 1.0)
        );

        let (t_x, t_p) = crate::kalman::predict(&x, &p, &a);
        x = t_x;
        p = t_p;

        let y = ( 
            (*measure, ),
            (*measure - last_measure, )
        );

        vec.push(
            (dt as u32, *measure as u32, x.0.0 as u32)
        );

        let (t_x, t_p) = crate::kalman::update(&x, &p, &y, &h, &r);    
        x = t_x;
        p = t_p;

        last_measure = *measure;      
        dt += 1.0;  
        
        
    }

    for p in vec {
        img.put_pixel(p.0, p.1, Rgb([255, 0, 0])); 
        img.put_pixel(p.0, p.2, Rgb([0, 0, 255]));                
    }

    img.save("score/simulated_kalman_filter.png")
}

fn sample_line_gen() -> Vec<f32> {
    let inc = 1.0;
    let step = 10;
    let mut x = 250.0;
    let mut line = Vec::new();

    for y in 1..501 {
        let variability = match y > 100 && y < 120 {
            true => rand::random::<i16>()/5000,
            false => rand::random::<i16>()/10000
        };
                
        line.push(
           x + variability as f32
        );
        if y%step == 0 {x = x + inc;}
    }

    line
}