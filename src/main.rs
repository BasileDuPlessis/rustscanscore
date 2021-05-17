use rand::prelude::*;
use image::{ImageBuffer, Rgb};


type M2x1 = (
    (f32,),
    (f32,)
);
type M2x2 = (
    (f32, f32),
    (f32, f32)
);

mod kalman;


fn main() {

    //tableau blanc spour l'image
    //un pixel noir est détecté sur une ligne à la colonne 0
    //->détecter des aberrations sur une ligne pas complètement droite à partir de colonne 1
    //prédiction => ligne attendue
    //générateur de point de mesure ligne/colonne +/- rand (très léger) + aberrations pendant n pixels à partir d'un seuil
    //Vk is the innovation or the measurement residual on time step k au carré comparé à une valeur prédéfinie = 10 fois la variance => aberration
    //mise à jour si ok
    //enregistrement d'un fichier image ligne prédite est décalée vers le bas de n pixels

    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        image::Rgb([255, 255, 255])
    });
    
    match simulator() {
        Err(e) => println!("{:?}", e),
        Ok(v) => {
            for p in v {
                img.put_pixel(p.0, p.1, Rgb([255, 0, 0])); 
                img.put_pixel(p.0, p.2, Rgb([0, 0, 255]));                
            }           
        }
    }

    img.save("rustscanscore.png");

}

fn simulator() -> Result<Vec<(u32, u32, u32)>, &'static str> {

    let measurements = sample_line_gen();
    let mut measurement_iter = measurements.iter();

    let mut last_measure= *measurement_iter.next().ok_or("Cannot init simulator with an empty vector")?; 

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

        let (t_x, t_p) = kalman::predict(&x, &p, &a);
        x = t_x;
        p = t_p;

        let y = ( 
            (*measure, ),
            (*measure - last_measure, )
        );

        vec.push(
            (dt as u32, *measure as u32, x.0.0 as u32)
        );

        let (t_x, t_p) = kalman::update(&x, &p, &y, &h, &r);    
        x = t_x;
        p = t_p;

        last_measure = *measure;      
        dt += 1.0;  
        
        
    }
    
    Ok(vec)
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
