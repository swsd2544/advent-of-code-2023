advent_of_code::solution!(5);

fn parse_dest_src_range_map(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|line| (line[0], line[1], line[2]))
        .collect()
}

fn get_destination(src: u32, dest_map: &[(u32, u32, u32)]) -> u32 {
    dest_map
        .iter()
        .find_map(|&(map_dest, map_src, map_range)| {
            if src >= map_src && src - map_src < map_range {
                Some(src - map_src + map_dest)
            } else {
                None
            }
        })
        .unwrap_or(src)
}

fn get_src(dest: u32, dest_map: &[(u32, u32, u32)]) -> u32 {
    dest_map
        .iter()
        .find_map(|&(map_dest, map_src, map_range)| {
            if dest >= map_dest && dest - map_dest < map_range {
                Some(dest - map_dest + map_src)
            } else {
                None
            }
        })
        .unwrap_or(dest)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut seeds = Vec::new();
    let mut seed_soil = Vec::new();
    let mut soil_fertilizer = Vec::new();
    let mut fertilizer_water = Vec::new();
    let mut water_light = Vec::new();
    let mut light_temperature = Vec::new();
    let mut temperature_humidity = Vec::new();
    let mut humidity_location = Vec::new();

    input.split("\n\n").for_each(|line| {
        if let Some(line) = line.strip_prefix("seeds: ") {
            seeds = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
        } else if let Some(line) = line
            .strip_prefix("seed-to-soil map:")
            .map(|line| line.trim())
        {
            seed_soil = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("soil-to-fertilizer map:")
            .map(|line| line.trim())
        {
            soil_fertilizer = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("fertilizer-to-water map:")
            .map(|line| line.trim())
        {
            fertilizer_water = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("water-to-light map:")
            .map(|line| line.trim())
        {
            water_light = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("light-to-temperature map:")
            .map(|line| line.trim())
        {
            light_temperature = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("temperature-to-humidity map:")
            .map(|line| line.trim())
        {
            temperature_humidity = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("humidity-to-location map:")
            .map(|line| line.trim())
        {
            humidity_location = parse_dest_src_range_map(line);
        }
    });

    seeds
        .iter()
        .map(|&seed| get_destination(seed, &seed_soil))
        .map(|soil| get_destination(soil, &soil_fertilizer))
        .map(|fertilizer| get_destination(fertilizer, &fertilizer_water))
        .map(|water| get_destination(water, &water_light))
        .map(|light| get_destination(light, &light_temperature))
        .map(|temperature| get_destination(temperature, &temperature_humidity))
        .map(|humidity| get_destination(humidity, &humidity_location))
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seeds = Vec::new();
    let mut seed_soil = Vec::new();
    let mut soil_fertilizer = Vec::new();
    let mut fertilizer_water = Vec::new();
    let mut water_light = Vec::new();
    let mut light_temperature = Vec::new();
    let mut temperature_humidity = Vec::new();
    let mut humidity_location = Vec::new();

    input.split("\n\n").for_each(|line| {
        if let Some(line) = line.strip_prefix("seeds: ") {
            seeds = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .map(|ch| (ch[0], ch[1]))
                .collect();
        } else if let Some(line) = line
            .strip_prefix("seed-to-soil map:")
            .map(|line| line.trim())
        {
            seed_soil = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("soil-to-fertilizer map:")
            .map(|line| line.trim())
        {
            soil_fertilizer = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("fertilizer-to-water map:")
            .map(|line| line.trim())
        {
            fertilizer_water = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("water-to-light map:")
            .map(|line| line.trim())
        {
            water_light = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("light-to-temperature map:")
            .map(|line| line.trim())
        {
            light_temperature = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("temperature-to-humidity map:")
            .map(|line| line.trim())
        {
            temperature_humidity = parse_dest_src_range_map(line);
        } else if let Some(line) = line
            .strip_prefix("humidity-to-location map:")
            .map(|line| line.trim())
        {
            humidity_location = parse_dest_src_range_map(line);
        }
    });

    (0..)
        .map(|location| get_src(location, &humidity_location))
        .map(|humidity| get_src(humidity, &temperature_humidity))
        .map(|temperature| get_src(temperature, &light_temperature))
        .map(|light| get_src(light, &water_light))
        .map(|water| get_src(water, &fertilizer_water))
        .map(|fertilizer| get_src(fertilizer, &soil_fertilizer))
        .map(|soil| get_src(soil, &seed_soil))
        .position(|seed| {
            seeds
                .iter()
                .any(|&(src, range)| seed >= src && seed - src < range)
        })
        .map(|idx| idx as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
