use std::ops::IndexMut;
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


    let mut staves = Vec::<Staff>::new();


    for (y, buff) in buffer_x.chunks(height).enumerate() {
        
        let staff_predictions = staves
            .iter()
            .map(|staff| staff.get_prediction(y))
            .collect::<Vec<(f32, f32)>>();

        let pixel_positions = buff
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(x, _)| x)
            .collect::<Vec<usize>>();

        let matches = pixel_positions
            .iter()
            .map(
                |p|
                match_position(&staff_predictions, &p)
            )
            .collect::<Vec<Option<usize>>>();

        let matched_pixels = matches.iter()
            .enumerate()
            .filter(|(_, opt)| opt.is_some())
            .map(|(i, opt)| (pixel_positions[i], opt.unwrap()))
            .collect::<Vec<(usize, usize)>>();

        for (s, xs) in group_by_equal_value(matched_pixels) {
            staves[s].add_pixels(xs, y);
        }
        

        let unmatched_pixels = matches
            .iter()
            .enumerate()    
            .filter(|(_, opt)| opt.is_none())
            .map(|(i, _)| pixel_positions[i])
            .collect::<Vec<usize>>();
        
        for xs in group_by_incremental_values(unmatched_pixels) {
            staves.push(Staff::new(xs, y))
        }

    }


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

fn group_by_incremental_values(vec: Vec<usize>) -> Vec<Vec<usize>> {
    let mut res:Vec<Vec<usize>> = Vec::new();

    if vec.len() > 0 {
        res.push(vec![vec[0]]);
    }    

    for s in vec.windows(2) {         
        match s {
            [current, next] if next-current==1 => 
                match res.last_mut() {
                    Some(last) => last.push(*next),
                    _ => ()
                },
            [_, next] => res.push(vec![*next]),
            _ => ()
        }
    }
    res
}

fn group_by_equal_value(vec: Vec<(usize, usize)>) -> Vec<(usize, Vec<usize>)> {
    let mut res:Vec<(usize, Vec<usize>)> = Vec::new();
       
    let mut iter = vec;
    
    iter.sort_by(
        |a, b| 
        a.1.partial_cmp(&b.1)
        .unwrap_or(std::cmp::Ordering::Equal)
    );

    if iter.len() > 0 {
        res.push((iter[0].1, vec![iter[0].0]));
    }

    for s in iter.windows(2) {
        match s {
            [current, next] if current.1==next.1 =>
                match res.last_mut() {
                    Some(last) => last.1.push(next.0),
                    _ => ()
                },
            [_, next] => res.push((next.1, vec![next.0])),
            _ => ()
         }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_equal_value_sort_values() {
        let vec = vec![
            (4, 3),
            (3, 4),
            (2, 3),
            (1, 4)
        ];
        let res= vec![
            (3, vec![4,2]),
            (4, vec![3,1])         
        ];

        assert_eq!(group_by_equal_value(vec), res);
    }

    #[test]
    fn test_group_by_equal_value_sort_values_2() {
        let vec = vec![
            (0, 4),
            (3, 4),
            (2, 4),
            (1, 3),
            (8, 4),
        ];
        let res= vec![
            (3, vec![1]),
            (4, vec![0,3,2,8])         
        ];

        assert_eq!(group_by_equal_value(vec), res);
    }

    #[test]
    fn test_group_by_equal_value_return_empty_if_empty() {
        let vec = Vec::new();
        let res= Vec::new();

        assert_eq!(group_by_equal_value(vec), res);
    }



    #[test]
    fn test_group_by_consecutive_value() {
        let vec = vec![0,2,3,4,6,7];
        let res= vec![
            vec![0],
            vec![2, 3, 4],
            vec![6, 7]
        ];

        assert_eq!(group_by_incremental_values(vec), res);
    }

    #[test]
    fn test_group_by_consecutive_value_handle_one_value() {
        let vec = vec![0];
        let res= vec![
            vec![0]
        ];

        assert_eq!(group_by_incremental_values(vec), res);
    }

    #[test]
    fn test_group_by_consecutive_value_return_empty_for_empty() {
        let vec = vec![];
        let res: Vec<Vec<usize>> = Vec::new();

        assert_eq!(group_by_incremental_values(vec), res);
    }

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
    buffer: Vec<(Vec<usize>, usize)>
}

impl Staff {
    fn new(xs: Vec<usize>, y: usize) -> Staff {   
        let mean: f32 = xs.iter().sum::<usize>() as f32 / xs.len() as f32;
        Staff {
            x: (
                (mean,),
                (0.0,)
            ),
            p: (
                (1.0, 0.0),
                (0.0, 1.0)
            ),
            buffer: vec![(xs, y)]
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
        let last_y = &self.buffer.last().unwrap().1;
        let a = (
            (1.0, (y - *last_y) as f32),
            (0.0, 1.0)
        );        
        let (t_x, _) = kalman::predict(&self.x, &self.p, &a);
        (t_x.0.0, t_x.1.0)
    }

    fn add_pixels(&mut self, xs: Vec<usize>, y: usize) {
        /*
        for t in self.buffer.iter_mut() {
            if t.1 == y {
                t.0.push(x);
            }
        }
         */
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