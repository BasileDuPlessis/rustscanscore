use std::usize;

mod staves;
mod kalman;



fn main() {
 

    /*
    for (id, y) in buffer_x.iter().enumerate() {
        buffer_y[buffer_id_swap(id, height, width)] = *y;
    }

    image::save_buffer("score_sample/binary_score.png", &buffer_y, width as u32, height as u32, image::ColorType::L8).unwrap();
    
    
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

#[cfg(test)]
mod tests {

    use super::*;

    fn prepare_img(img: &str) -> (Vec<u8>, usize, usize) {
        let img = image::open(img).unwrap();
        let img_gray = img.into_luma8();
    
        let width = img_gray.width() as usize;
        let height = img_gray.height() as usize;
        let buffer_size = width * height;
    
    
        let mut buffer_vertical = vec![255; buffer_size];
    
      
        for id_horizontal in 0..(buffer_size-1) {
            let id_vertical = buffer_id_swap(id_horizontal, width, height);
            buffer_vertical[id_vertical] = *img_gray.get(id_horizontal).unwrap_or(&255);
        }

        (buffer_vertical, width, height)
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

    #[test]
    fn test_one_full_line_get_one_staff_with_10_items() {
        let (buffer, _, height) = prepare_img("score_sample/single_line_top.png");

        let staves = staves::detect_staves(buffer, height);

        assert_eq!(staves.len(), 1);
        assert_eq!(staves[0].buffer.len(), 10);
    }

    #[test]
    fn test_full_black() {
        let (buffer, _, height) = prepare_img("score_sample/full_black.png");

        let staves = staves::detect_staves(buffer, height);

        println!("{:?}", staves);
    }

}
