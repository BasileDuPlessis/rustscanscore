use std::usize;
use std::cmp::Ordering;

mod kalman;
mod simulator;


fn main() {

    let img = image::open("score_sample/score_sample2.png").unwrap();
    let gray = img.into_luma8();

    let width = gray.width() as usize;
    let height = gray.height() as usize;
    let buffer_size = width * height;


    let mut buffer_x = vec![255; buffer_size];
    let mut buffer_y = vec![255; buffer_size];

  
    for id_x in 1..(buffer_size-1) {
        let prev_id_y = buffer_id_swap(id_x-1, height, width);
        let next_id_y = buffer_id_swap(id_x+1, height, width);

        let diff:i32 = 
            *gray.get(prev_id_y).unwrap() as i32
            -
            *gray.get(next_id_y).unwrap() as i32;

        if diff > 40  {
            buffer_x[id_x] = 0;
        }
    }

 /*
    buffer_height[1265] = 0;
    buffer_height[1413] = 0;
    buffer_height[1560] = 0;
    buffer_height[1855] = 0;
    buffer_height[3777] = 0;
    buffer_height[3926] = 0;
    buffer_height[4075] = 0;
    buffer_height[4224] = 0;
    buffer_height[4373] = 0;
*/


    let staves = Vec::<Staff>::new();


    for (y, buff) in buffer_x.chunks(height).enumerate() {
        
        let predictions = staves
            .iter()
            .map(|staff| staff.get_prediction(y))
            .collect::<Vec<(f32, f32)>>();

        let positions = buff
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(x, _)| x);

        let matches = positions
            .map(
                |p|
                match_position(&predictions, &p)
            );
        
        for (key, res) in matches.enumerate() {
            match res {
                None => (), //staves.push()
                Some(id) => () //staves[id].add( (position[k], y) )
            }
        }

    }



/*
    while let Some(point) = iter.next() {

        if *point.1 != 0 {continue;}
        
        let pixel = (point.0 % height, point.0 / height);

        
        staves.iter_mut().for_each(|s| s.set_distance(pixel));
        staves.sort();


        match staves.iter_mut().find(|s| s.check_pixel(pixel)) {
            None => staves.push(Staff::new(pixel)),
            Some(staff) => staff.add_pixel(pixel)
        }

        loop {
            match iter.peek() {
                Some(p) if *p.1==0 => iter.next(),
                _ => break
            };
        }

    }
   
    for s in staves.iter() {
        println!("{:?}\n", s);
       if s.buffer.len() > width / 2 && s.active {
            for mut idx in s.buffer.iter().map(|t| t.1*height + t.0) {               
                while buffer_height[idx] == 0 {
                    buffer_height[idx] = 120;
                    idx = idx+1;
                    break;
                }
            }
       }
    }
    */
  
    //println!("{:?}", staves);
    for (id, y) in buffer_x.iter().enumerate() {
        buffer_y[buffer_id_swap(id, height, width)] = *y;
    }

    image::save_buffer("score_sample/binary_score.png", &buffer_y, width as u32, height as u32, image::ColorType::L8).unwrap();
    
    /*
    match simulator::line() {
        Err(e) => println!("{:?}", e),
        _ => ()
    };
    */
}

fn buffer_id_swap(id: usize, a: usize, b: usize) -> usize {
    let dim1 = id % a;
    let dim2 = id / a;
    dim1 * b + dim2
}

fn match_position(predictions: &Vec<(f32, f32)>, position: &usize) -> Option<usize> {
    let mut result = predictions
        .iter()
        .enumerate()
        .map(|(id, t)| (id, (*position as f32 - t.0.round()).abs(), t.1))
        .filter(|(_, diff, _)| *diff <= 1.0)
        .collect::<Vec<(usize, f32, f32)>>();
   
    result.sort_by(
        |a,b| 
        a.2
            .partial_cmp(&b.2)
            .unwrap_or(std::cmp::Ordering::Equal)
    );

    match result.first() {
        None => None,
        Some(prediction) => Some(prediction.0)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_position_foster_horizontality() {
        let predictions = vec![
            (1.0, 1.0),
            (0.0, 0.0)        
        ];
        let position = 1;
        assert_eq!(match_position(&predictions, &position), Some(1));
    }
    #[test]
    fn test_match_position_round_predicition() {
        let predictions = vec![
            (0.49, 1.0)
        ];
        let position = 2;
        assert_eq!(match_position(&predictions, &position), None);
        let predictions = vec![
            (0.50, 1.0)
        ];
        let position = 2;
        assert_eq!(match_position(&predictions, &position), Some(0));
    }
    #[test]
    fn test_match_position_follow_sorting_in_eq_condition() {
        let pred1 = vec![        
            (2.0, 0.0),
            (0.0, 0.0)
        ];
        let pred2 = vec![        
            (0.0, 0.0),
            (2.0, 0.0)
        ];
        let position = 1;
        assert_eq!(match_position(&pred1, &position), match_position(&pred2, &position));
    }


    #[test]
    fn test_buffer_idx_swap_first_one_last_one_keep_same() {
        let height = 5;
        let width = 10;
        let idx_first = 0;
        let idx_last = 49;
        assert_eq!(buffer_id_swap(idx_first, width, height), idx_first);
        assert_eq!(buffer_id_swap(idx_last, height, width), idx_last);
    }
    #[test]
    fn test_buffer_idx_swap_point_translation_is_reversible() {
        let height = 5;
        let width = 10;
        let idx_height = 9;
        let idx_width = 45;
        assert_eq!(buffer_id_swap(idx_height, width, height), idx_width);
        assert_eq!(buffer_id_swap(idx_width, height, width), idx_height);
    }
}



#[derive(Debug)]
struct Staff {
    x: kalman::M2x1,
    p: kalman::M2x2,
    buffer: Vec<(usize, Vec<usize>)>
}
/*
impl Ord for Staff {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Staff {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Staff {
}

impl PartialEq for Staff {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
*/

impl Staff {
    fn new() -> Staff {        
        Staff {
            x: (
                (0.0,),
                (0.0,)
            ),
            p: (
                (1.0, 0.0),
                (0.0, 1.0)
            ),
            buffer: vec![]
        }
    }
    
    const H:kalman::M2x2 = (
        (1.0, 0.0),
        (0.0, 1.0)
    );
    const R:kalman::M2x2 = (
        (1.0, 0.0),
        (0.0, 1.0)
    );


    fn get_prediction(&self, y: usize) -> (f32, f32) {
        let last_y = &self.buffer.last().unwrap().0;
        let a = (
            (1.0, (y - *last_y) as f32),
            (0.0, 1.0)
        );        
        let (t_x, _) = kalman::predict(&self.x, &self.p, &a);
        (t_x.0.0, t_x.1.0)
    }

/* 
    fn add_pixel(&mut self, pixel: (usize, usize)) {
    
        let last_pixel = &self.buffer.last().unwrap();

        let a = (
            (1.0, pixel.1 as f32 - last_pixel.1 as f32),
            (0.0, 1.0)
        );
        let (t_x, t_p) = kalman::predict(&self.x, &self.p, &a);
        
        let speed = (pixel.0 as f32 - last_pixel.0 as f32) / (pixel.1 as f32 - last_pixel.1 as f32);

       

        let y = ( 
            (pixel.0 as f32, ),
            (speed, )
        );

        let (t_x, t_p) = kalman::update(&t_x, &t_p, &y, &Staff::H, &Staff::R);   
        
        self.x = t_x;
        self.p = t_p;

        self.buffer.push(pixel);


        if self.x.1.0.abs() > 0.5 && self.buffer.len() > 2 {self.active = false;}
    }
    */
}