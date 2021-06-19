#[derive(Debug)]
struct Prediction {    
    from_y: f32,
    x: f32,
    bias: f32
}

#[derive(Debug)]
pub struct Staff {
    x: crate::kalman::M2x1,
    p: crate::kalman::M2x2,
    pub buffer: Vec<(Vec<usize>, usize)>
}

impl Staff {

    fn new(xs: Vec<usize>, y: usize) -> Staff {   
        
        let mean = Staff::get_mean(&xs).unwrap();

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
    
    const H:crate::kalman::M2x2 = (
        (1.0, 0.0),
        (0.0, 1.0)
    );
    const R:crate::kalman::M2x2 = (
        (1.0, 0.0),
        (0.0, 1.0)
    );


    fn get_prediction(&self, y: usize) -> Prediction {
        let last_y = self.buffer.last().unwrap().1 as f32;
        
        let a = (
            (1.0, y as f32 - last_y),
            (0.0, 1.0)
        );

        let (t_x, _) = crate::kalman::predict(&self.x, &self.p, &a);

        Prediction {
            from_y: last_y,
            x: t_x.0.0,
            bias: t_x.1.0
        }
                
    }

    fn push_pixels(&mut self, xs: Vec<usize>, y: usize) {
        
        let default = (xs.clone(), y.clone());

        let last_pixels = self.buffer.last().unwrap_or(&default);

        let x_mean = Staff::get_mean(&xs).unwrap();
        let last_x_mean = Staff::get_mean(&last_pixels.0).unwrap();

        let a = (
            (1.0,  y as f32 - last_pixels.1 as f32),
            (0.0, 1.0)
        );

        let (t_x, t_p) =
        crate::kalman::predict(&self.x, &self.p, &a);
        
        let speed = 
            x_mean - last_x_mean
            /
            (y as f32 - last_pixels.1 as f32);

        let measure = ( 
            (x_mean as f32, ),
            (speed, )
        );

        let (t_x, t_p) =
        crate::kalman::update(&t_x, &t_p, &measure, &Staff::H, &Staff::R);   
        
        self.x = t_x;
        self.p = t_p;

        self.buffer.push((xs, y));

    }

    fn get_mean(xs: &Vec<usize>) -> Option<f32> {
        let len = xs.len() as f32;
        match xs.as_slice() {
            [] => None,
            _ => Some(
                (xs.iter().sum::<usize>() as f32 + len * 0.5)
                /
                len
            )
        }        
    }

}

pub fn detect_staves(buffer_vertical:Vec<u8>, height: usize) -> Vec<Staff> {
    let mut staves = Vec::<Staff>::new();

    for (y, buff) in buffer_vertical.chunks(height).enumerate() {

        let pixel_positions = buff
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(x, _)| x + 1)
            .collect::<Vec<usize>>();

        if pixel_positions.len() == 0 {continue;}

        let y = y + 1;
        
        let staff_predictions = staves
            .iter()
            .map(|staff| staff.get_prediction(y))
            .collect::<Vec<Prediction>>();

        let matches = pixel_positions
            .iter()
            .map(
                |x|
                match_position(&staff_predictions, x, &y)
            )
            .collect::<Vec<Option<usize>>>();

        let matched_pixels = matches.iter()
            .enumerate()
            .filter(|(_, opt)| opt.is_some())
            .map(|(i, opt)| (pixel_positions[i], opt.unwrap()))
            .collect::<Vec<(usize, usize)>>();

        for (s, xs) in group_by_equal_value(matched_pixels) {
            staves[s].push_pixels(xs, y);
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

    staves
    
}

fn match_position(predictions: &Vec<Prediction>, x: &usize, y: &usize) -> Option<usize> {
    let mut result = predictions
        .iter()
        .enumerate()
        .filter(|(_, pred)| (*x as f32 + 0.5 - pred.x).abs() <= 1.2)
        .map(
            |(id, pred)|
            (id, *y as f32 - pred.from_y, pred.bias))
        .collect::<Vec<(usize, f32, f32)>>();

    result.sort_by(
        |a,b| 
        a.1
            .partial_cmp(&b.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(
                a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal)
            )
    );

    match result.first() {
        None => None,
        Some(r) => Some(r.0)
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
    fn test_match_position_use_pixel_center() {
        let predictions = vec![
            Prediction {x: 5.0, from_y: 1.0, bias: 0.0},       
        ];
        let x = 4;
        let y = 2;
        assert_eq!(match_position(&predictions, &x, &y), Some(0));
        let x = 6;
        let y = 2;
        assert_eq!(match_position(&predictions, &x, &y), None);
    }

    #[test]
    fn test_match_position_foster_continuity_and_horizontality() {
        let predictions = vec![
            Prediction {x: 1.0, from_y: 1.0, bias: 1.0},
            Prediction {x: 2.0, from_y: 3.0, bias: 0.0},        
        ];
        let x = 1;
        let y = 4;
        assert_eq!(match_position(&predictions, &x, &y), Some(1));
    }
     
  
    #[test]
    fn test_match_position_disadvantage_distant_staff() {
        let predictions = vec![
            Prediction {x: 1., from_y: 1.0, bias: 0.0},    
            Prediction {x: 2., from_y: 2.0, bias: 0.5}
        ];
        let x = 1;
        let y = 3;
        assert_eq!(match_position(&predictions, &x, &y), Some(1));
    }
    #[test]
    fn test_match_position_follow_sorting_in_eq_condition() {
        let pred1 = vec![      
            Prediction {x: 2., from_y: 1.0, bias: 0.0},    
            Prediction {x: 1.5, from_y: 1.0, bias: 0.0}  
        ];
        let pred2 = vec![        
            Prediction {x: 1.5, from_y: 1.0, bias: 0.0},
            Prediction {x: 2., from_y: 1.0, bias: 0.0}
        ];
        let x = 2;
        let y = 2;
        assert_eq!(match_position(&pred1, &x, &y), Some(0));
        assert_eq!(match_position(&pred2, &x, &y), Some(0));
    }

    #[test]
    fn test_get_mean() {
        let buffer:Vec<usize> = vec![5,7,9];

        assert_eq!(Staff::get_mean(&buffer), Some(7.5));
        let buffer:Vec<usize> = vec![1,2,3,4];

        assert_eq!(Staff::get_mean(&buffer), Some(3.0));
    }

   
}

