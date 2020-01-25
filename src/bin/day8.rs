use std::env::args;
use std::fs;

struct Layer {
    pixels: Vec<Vec<char>>,
}

impl Layer {
    fn parse(pixels: &[char], width: usize) -> Self {
        // XXX
        Layer {
            pixels: pixels
                .chunks(width)
                .map(|x| x.iter().cloned().collect())
                .collect(),
        }
    }

    fn count_color_in_row(row: &Vec<char>, color: char) -> usize {
        row.iter()
            .fold(0, |acc, char| acc + if *char == color { 1 } else { 0 })
    }

    fn count_color(&self, color: char) -> usize {
        self.pixels
            .iter()
            .fold(0, |acc, row| acc + Layer::count_color_in_row(row, color))
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn find_layer_with_fewest_of_color(&self, color: char) -> usize {
        let mut lowest = (0, std::usize::MAX);
        for i in 0..self.layers.len() {
            let this = self.layers[i].count_color(color);
            if this < lowest.1 {
                lowest = (i, this);
            }
        }
        lowest.0
    }

    fn parse(file: &str, width: usize, height: usize) -> Self {
        let input = fs::read_to_string(file).unwrap();
        // Make sure there's exactly one line and it has a good number of characters.
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 1);
        let pixels: Vec<char> = lines[0].trim().chars().collect();
        assert_eq!(pixels.len() % (width * height), 0);

        let layers = pixels
            .chunks(width * height)
            .map(|chunk| Layer::parse(chunk, width))
            .collect();
        Image { layers }
    }

    fn layer(&self, i: usize) -> &Layer {
        &self.layers[i]
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let width = args()
        .nth(2)
        .expect("no width given")
        .parse::<usize>()
        .unwrap();
    let height = args()
        .nth(3)
        .expect("no height given")
        .parse::<usize>()
        .unwrap();
    let mut img = Image::parse(&filename, width, height);
    let zeroes_layer_idx = img.find_layer_with_fewest_of_color('0');
    let zeroes_layer = img.layer(zeroes_layer_idx);
    dbg!(zeroes_layer.count_color('1') * zeroes_layer.count_color('2'));
}
