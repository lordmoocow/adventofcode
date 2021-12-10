const DATA_WIDTH: usize = 12;

pub struct Diagnostics {
    report: Vec<usize>,
}

impl Diagnostics {
    pub fn from(report: Vec<usize>) -> Self {
        Self { report }
    }

    pub fn get_power_consumption(&self) -> usize {
        let gamma = most_common_bits(&self.report);
        // epsilon is inverse of gamma
        // we only want to flip the bits for the width of our data so that we get the correct decimal value
        //gamma ^ 0b_1111_1111_1111
        let epsilon = gamma ^ !(!0 << DATA_WIDTH);
        gamma * epsilon
    }

    pub fn get_life_support_rating(&self) -> usize {
        let o2 = calculate_rating(&self.report, most_common_bits);
        let co2 = calculate_rating(&self.report, least_common_bits);
        o2 * co2
    }
}

// Determine most common bits of a data set
fn most_common_bits(data: &[usize]) -> usize {
    normalise_bit_count(data, |count, threshold| count >= threshold)
}

// Determine least common bits of a data set
fn least_common_bits(data: &[usize]) -> usize {
    normalise_bit_count(data, |count, threshold| count < threshold)
}

// Count number of high bits in a data set
fn count_bits(data: &[usize]) -> [usize; DATA_WIDTH] {
    let mut bit_counts = [0; DATA_WIDTH];
    for reading in data {
        for i in 0..DATA_WIDTH {
            if reading & (1 << i) != 0 {
                bit_counts[i] += 1;
            }
        }
    }
    bit_counts
}

// Counts high bits across the data set and normalises according to the rule
fn normalise_bit_count<F>(data: &[usize], rule: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let mut result = 0;
    let bit_counts = count_bits(data);
    let threshold = (data.len() as f64 / 2.0).round() as usize;

    for (i, count) in bit_counts.into_iter().enumerate() {
        if rule(count, threshold) {
            // left-shift 1 bit by the current index to the position of the bit we want to set high
            result |= 1 << i;
        }
    }

    result
}

fn calculate_rating<F>(data: &[usize], rule: F) -> usize
where
    F: Fn(&[usize]) -> usize,
{
    // Copy the data into a mutable vector which can be filtered down as we go
    let mut data = data.to_vec();
    for n in 0..DATA_WIDTH {
        // create a bit mask for the current position we want to inspect
        let mask = 1 << (DATA_WIDTH - n - 1);
        let target = rule(&data);
        // filter data to the bits matching the rule
        // use the mask to compare the correct bits
        data.retain(|reading| reading & mask == target & mask);

        // if we only have one reading left, that is the rating
        if data.len() == 1 {
            return data[0];
        }
    }

    0
}
