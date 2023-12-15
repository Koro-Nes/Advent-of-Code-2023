use std::{ops::Range, io::{BufReader, BufRead}, fs::File};

#[derive(Debug)]
struct ConversionMap {
    source_range: Range<u128>,
    destination_range: Range<u128>,
    map_type: String,
}

impl ConversionMap {
    fn init(range_size: u128, source_start: u128, destination_start: u128, t: &String) -> ConversionMap {
        return ConversionMap { source_range: source_start..source_start+range_size-1, 
            destination_range: destination_start..destination_start+range_size-1, map_type: t.clone() }
    }
    fn get_num(&self, n: u128) -> u128 {

        if inclusive_contains(&self.source_range, n) {
            let res = self.destination_range.start + (n - self.source_range.start);
            return res;
        }
        return n;
    }
}

#[derive(Debug)]
struct TotalMaps {
   seeds: Vec<u128>, 
   seed_to_soil: Vec<ConversionMap>,
   soil_to_fertilizer: Vec<ConversionMap>,
   ferilizer_to_water: Vec<ConversionMap>,
   water_to_light: Vec<ConversionMap>,
   light_to_temperature: Vec<ConversionMap>,
   temperature_to_humidity: Vec<ConversionMap>,
   humidity_to_location: Vec<ConversionMap>,
}

impl TotalMaps {
    fn init( (seeds, maps): (Vec<u128>, Vec<ConversionMap>) ) -> TotalMaps {
        let mut seed_to_soil: Vec<ConversionMap> = Vec::new();
        let mut soil_to_fertilizer: Vec<ConversionMap> = Vec::new();
        let mut ferilizer_to_water: Vec<ConversionMap> = Vec::new();
        let mut water_to_light: Vec<ConversionMap> = Vec::new();
        let mut light_to_temperature: Vec<ConversionMap> = Vec::new();
        let mut temperature_to_humidity: Vec<ConversionMap> = Vec::new();
        let mut humidity_to_location: Vec<ConversionMap> = Vec::new();
        for m in maps {
            match m.map_type.as_str() {
                "ss" => seed_to_soil.push(m),
                "sf" => soil_to_fertilizer.push(m),
                "fw" => ferilizer_to_water.push(m),
                "wl" => water_to_light.push(m),
                "lt" => light_to_temperature.push(m),
                "th" => temperature_to_humidity.push(m),
                "hl" => humidity_to_location.push(m),
                _ => panic!("Error pushing ConversionMap to TotalMap."),
            }
        }
        return TotalMaps { seeds, seed_to_soil, soil_to_fertilizer, ferilizer_to_water, 
            water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location };
    }

    fn get_locations(&self) -> Vec<u128> {
        let mut res: Vec<u128> = Vec::new();
        for seed in &self.seeds {
            let mut curr_num: u128;
            curr_num = get_num_from_vec(&self.seed_to_soil, *seed); 
            curr_num = get_num_from_vec(&self.soil_to_fertilizer, curr_num);
            curr_num = get_num_from_vec(&self.ferilizer_to_water, curr_num);
            curr_num = get_num_from_vec(&self.water_to_light, curr_num);
            curr_num = get_num_from_vec(&self.light_to_temperature, curr_num);
            curr_num = get_num_from_vec(&self.temperature_to_humidity, curr_num);
            curr_num = get_num_from_vec(&self.humidity_to_location, curr_num);
            res.push(curr_num);
        } 
        return res;
    }
}

fn get_num_from_vec(v: &Vec<ConversionMap>, n: u128) -> u128 {
    let mut res_vec: Vec<u128> = Vec::new();
    for m in v {
        let res = m.get_num(n);
        res_vec.push(res); 
    }
    for e in &res_vec {
        if *e != n {
            return *e;
        }
    }
    return res_vec[0];
}

fn main() {

    let file: File = File::open("./input.txt").unwrap();
    let buf: BufReader<File> = BufReader::new(file);
    let input_strings = read_input(buf);
    let parsed_maps = parse_input(input_strings);
    let total_map = TotalMaps::init(parsed_maps);
    let res_locations = total_map.get_locations();
    let min_location = res_locations.iter().min().unwrap();
    println!("Min value: {}", min_location);

}

fn read_input(buf: BufReader<File>) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    for l in buf.lines() {
        strings.push(l.unwrap());
    }
    return strings;
}

fn parse_input(vec: Vec<String>) -> (Vec<u128>, Vec<ConversionMap>) {
    let mut t: String = String::from("u");
    let mut maps: Vec<ConversionMap> = Vec::new();
    let mut seeds: Vec<u128> = Vec::new();

    for line in vec {
        let seperated_strings: Vec<&str> = line.trim().split(" ").collect();
        let identifier = seperated_strings[0];
        let num = identifier.parse::<u32>();
        if num.is_err() {
            t = match seperated_strings[0].trim() {
                "seeds:" => String::from("s"),
                "seed-to-soil" => String::from("ss"),
                "soil-to-fertilizer" => String::from("sf"),
                "fertilizer-to-water" => String::from("fw"),
                "water-to-light" => String::from("wl"),
                "light-to-temperature" => String::from("lt"),
                "temperature-to-humidity" => String::from("th"),
                "humidity-to-location" => String::from("hl"),
                _ => t,
            };

            if t == String::from("s") {
                if seperated_strings.len() > 1 {
                    for i in 1..seperated_strings.len() {
                        seeds.push(seperated_strings[i].parse().unwrap());
                    }
                }
            }

        } else {
            let seperated_num_strings: Vec<&str> = line.trim().split(" ").collect();
            let curr_map = ConversionMap::init(
            seperated_num_strings[2].parse().unwrap(),
            seperated_num_strings[1].parse().unwrap(), 
        seperated_num_strings[0].parse().unwrap(), &t);
            maps.push(curr_map);
        }
    }
    return (seeds, maps);
}

fn inclusive_contains(range: &Range<u128>, n: u128) -> bool {
    return n >= range.start && n <= range.end;
}