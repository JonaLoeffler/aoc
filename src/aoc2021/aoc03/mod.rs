static INPUT: &'static str = include_str!("./input");

fn read() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| {
            l.split("")
                .map(|c| c.parse::<u32>())
                .filter_map(|c| c.ok())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn sum(numbers: Vec<Vec<u32>>) -> Vec<u32> {
    let row = numbers.get(0).unwrap();

    let mut sum: Vec<u32> = (0..(row.len())).map(|_| 0).collect();

    numbers.iter().for_each(|n| {
        n.iter().enumerate().for_each(|(i, n)| sum[i] += n);
    });

    sum
}

fn most_common(numbers: Vec<Vec<u32>>) -> Vec<u32> {
    let most_common: Vec<u32> = sum(numbers.clone())
        .iter()
        .map(|s| {
            let count = numbers.len().try_into().unwrap();

            if s * 2 < count {
                1
            } else {
                0
            }
        })
        .collect();

    most_common
}

fn least_common(numbers: Vec<Vec<u32>>) -> Vec<u32> {
    let least_common: Vec<u32> = most_common(numbers).iter().map(|g| 1 - g).collect();

    least_common
}

fn binary_vec_to_int(numbers: Vec<u32>) -> i32 {
    isize::from_str_radix(
        &numbers
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap()
    .try_into()
    .unwrap()
}

fn filter(numbers: Vec<Vec<u32>>, filter: &dyn Fn(Vec<Vec<u32>>) -> Vec<u32>) -> Vec<u32> {
    let mut tmp: Vec<Vec<u32>> = numbers.to_owned();

    for i in 0..(tmp.len()) {
        let mcb = filter(tmp.clone());

        tmp = tmp
            .iter()
            .filter(|n| n.get(i).unwrap() == mcb.get(i).unwrap())
            .map(|i| i.clone())
            .collect();

        if tmp.len() == 1 {
            return tmp.get(0).unwrap().to_vec();
        }
    }

    panic!("Nothing left after filter!");
}

pub fn one() -> i32 {
    let gamma = most_common(read());
    let epsilon = least_common(read());

    let gamma = binary_vec_to_int(gamma);
    let epsilon = binary_vec_to_int(epsilon);

    gamma * epsilon
}

pub fn two() -> i32 {
    let oxygen = binary_vec_to_int(filter(read(), &most_common));
    let co2 = binary_vec_to_int(filter(read(), &least_common));

    oxygen * co2
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        assert_eq!(4139586, super::one())
    }

    #[test]
    fn test_two() {
        assert_eq!(1800151, super::two())
    }
}
