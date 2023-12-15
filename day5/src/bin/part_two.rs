use std::{io::{BufReader, BufRead}, fs::File, ops::Range, time::Instant};
use rayon::prelude::*;

#[derive(Debug)]
struct Map {
    maps: Vec<PartialMap>,
}

#[derive(Debug)]
struct PartialMap {
    source_range: Range<u128>,
    destination_range: Range<u128>
}

impl Map {
    #[inline]
    fn get_mapping(&self, n: u128) -> u128 {
        let mut res: u128 = n;
        for pm in &self.maps {
            if pm.source_range.contains(&n) {
                res = n - pm.source_range.start + pm.destination_range.start;
                break;
            }
        }
        return res;
    }
}

fn main() {
    let mut now = Instant::now();

    let file: File = File::open("./input.txt").unwrap();
    let buf = BufReader::new(file);

    // parse input
    let mut map_type = "";
    let mut seeds: Vec<u128> = Vec::new();
    let mut ss: Vec<Vec<u128>> = Vec::new();
    let mut sf: Vec<Vec<u128>> = Vec::new();
    let mut fw: Vec<Vec<u128>> = Vec::new();
    let mut wl: Vec<Vec<u128>> = Vec::new();
    let mut lt: Vec<Vec<u128>> = Vec::new();
    let mut th: Vec<Vec<u128>> = Vec::new();
    let mut hl: Vec<Vec<u128>> = Vec::new();

    println!("Starting to parse document.");
    for line in buf.lines() {
        let curr_str = line.unwrap();
        let curr_line: Vec<&str> = curr_str.trim().split(" ").collect();
        map_type = match curr_line[0] {
            "seeds:" => "s",
            "seed-to-soil" => "ss",
            "soil-to-fertilizer" => "sf",
            "fertilizer-to-water" => "fw",
            "water-to-light" => "wl",
            "light-to-temperature" => "lt",
            "temperature-to-humidity" => "th",
            "humidity-to-location" => "hl",
            _ => map_type,
        };
        match map_type { 
            "s" => {
                    for i in 1..curr_line.len() {
                    seeds.push(curr_line[i].parse().unwrap());
                    }
            },
            "ss" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    ss.push(map);
                }
            },
            "sf" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    sf.push(map);
                }
            },
            "fw" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    fw.push(map);
                }
            },
            "wl" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    wl.push(map);
                }
            },
            "lt" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    lt.push(map);
                }
            },
            "th" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    th.push(map);
                }
            },
            "hl" => {
                if curr_line[0].parse::<u128>().is_ok() {
                    let mut map: Vec<u128> = Vec::new();
                    for n in curr_line {
                        map.push(n.parse().unwrap());
                    }
                    hl.push(map);
                }
            }
            _ => panic!(),
        }
    }
    let maps = create_maps(ss, sf, fw, wl, lt, th, hl);
    let seed_ranges = create_seed_ranges(seeds); 
    println!("Finished parsing in {:?}", now.elapsed());
    now = Instant::now();
    let result = get_min_location(seed_ranges, maps);
    println!("Finished in: {:?}", now.elapsed());
    println!("Result: {result}");
    
}

#[inline]
fn create_maps(
    ss: Vec<Vec<u128>>,
    sf: Vec<Vec<u128>>,
    fw: Vec<Vec<u128>>,
    wl: Vec<Vec<u128>>,
    lt: Vec<Vec<u128>>,
    th: Vec<Vec<u128>>,
    hl: Vec<Vec<u128>>,)
    -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    
    maps.push(Map { maps: get_partial_maps(ss) });
    maps.push(Map { maps: get_partial_maps(sf) });
    maps.push(Map { maps: get_partial_maps(fw) });
    maps.push(Map { maps: get_partial_maps(wl) });
    maps.push(Map { maps: get_partial_maps(lt) });
    maps.push(Map { maps: get_partial_maps(th) });
    maps.push(Map { maps: get_partial_maps(hl) });
 

    return maps;
}

#[inline]
fn get_partial_maps(v: Vec<Vec<u128>>) -> Vec<PartialMap> {
    let mut partials: Vec<PartialMap> = Vec::new();
    for p in v {
        partials.push(vec_to_partial_map(p));
    }
    return partials;
}

#[inline]
fn vec_to_partial_map(v: Vec<u128>) -> PartialMap {
    return PartialMap { source_range: v[1]..v[1]+v[2], destination_range: v[0]..v[0]+v[2] }
}

#[inline]
fn get_location(maps: &Vec<Map>, n: u128) -> u128 {
    let mut res: u128 = n;
    for pm in maps {
        res = pm.get_mapping(res)
    }
    return res; 
}

#[inline]
fn create_seed_ranges(v: Vec<u128>) -> Vec<Range<u128>> {
    let mut i: usize = 0;
    let mut res: Vec<Range<u128>> = Vec::new();
    while i < v.len() {
        res.push(v[i]..v[i]+v[i+1]);
        i += 2;
    }
    return res;
}

#[inline]
fn get_min_location(seeds: Vec<Range<u128>>, maps: Vec<Map>) -> u128 {
    let results: Vec<u128> = seeds.par_iter().map(|x| seed_range_to_min(&x, &maps)).collect();
    return *results.iter().min().unwrap();
}

fn seed_range_to_min(s: &Range<u128>, maps: &Vec<Map>) -> u128 {
    let mut min: u128 = u128::MAX;
    for n in s.to_owned() {
        let loc = get_location(&maps, n);
        if loc < min {
            min = loc;
        }
    }
    return min;
}