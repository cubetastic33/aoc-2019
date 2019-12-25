const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

// Serializer function
pub fn input_generator(image_data: String) -> Vec<[char; IMAGE_WIDTH * IMAGE_HEIGHT]> {
    let mut image = Vec::new();
    for layer in image_data.chars().collect::<Vec<_>>().chunks(IMAGE_WIDTH * IMAGE_HEIGHT) {
        let mut image_layer = [' '; IMAGE_WIDTH * IMAGE_HEIGHT];
        for (i, pixel) in layer.iter().enumerate() {
            image_layer[i] = *pixel;
        }
        image.push(image_layer);
    }
    image
}

// Solver function for part 1
pub fn find_checksum(image: Vec<[char; IMAGE_WIDTH * IMAGE_HEIGHT]>) -> usize {
    let mut checksum = 0;
    let mut least_number_of_zeros = IMAGE_WIDTH * IMAGE_HEIGHT;
    for layer in image {
        let number_of_zeros = layer.iter().filter(|x| **x == '0').count();
        if number_of_zeros < least_number_of_zeros {
            least_number_of_zeros = number_of_zeros;
            checksum = layer.iter().filter(|x| **x == '1').count() * layer.iter().filter(|x| **x == '2').count();
        }
    }
    checksum
}

// Solver function for part 2
pub fn decode_image(image: Vec<[char; IMAGE_WIDTH * IMAGE_HEIGHT]>) -> String {
    let mut decoded_image = vec![" "; IMAGE_WIDTH * IMAGE_HEIGHT];
    for layer in image {
        for (i, pixel) in layer.iter().enumerate() {
            if *pixel != '2' && decoded_image[i] == " " {
                decoded_image[i] = match pixel {
                    '0' => "█",
                    '1' => "▒",
                    _ => unreachable!()
                };
            }
        }
    }
    let mut printable_image = String::new();
    for row in decoded_image.chunks(IMAGE_WIDTH) {
        printable_image += &(String::from("\n") + &row.join(""));
    }
    printable_image
}
