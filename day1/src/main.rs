use aoc_util::anyhow;
use std::fs::read_to_string;

fn main() -> Result<(), anyhow::Error> {
    let text = read_to_string("input")?;
    let res = part2(&text)?;
    println!("the answer is: {res}");
    Ok(())
}
fn get_vec_ids(text: &str) -> anyhow::Result<Vec<(i32, i32)>> {
    text.lines()
        .map(|s| {
            let mut s = s.split_whitespace();
            if let (Some(s1), Some(s2)) = (s.next(), s.next()) {
                let n1 = s1.parse::<i32>()?;
                let n2 = s2.parse::<i32>()?;
                Ok((n1, n2))
            } else {
                Err(anyhow::anyhow!("failed to parse to text right"))
            }
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()
}
fn part1(text: &str) -> anyhow::Result<i32> {
    let mut v1 = vec![];
    let mut v2 = vec![];

    get_vec_ids(text)?.into_iter().for_each(|(n1, n2)| {
        v1.push(n1);
        v2.push(n2);
    });
    v1.sort();
    v2.sort();
    let sum = v1
        .iter()
        .zip(v2.iter())
        .map(|(n1, n2)| (n2 - n1).abs())
        .sum();
    Ok(sum)
}
fn part2(text: &str) -> anyhow::Result<i32> {
    let mut v1 = vec![];
    let mut v2 = vec![];

    get_vec_ids(text)?.into_iter().for_each(|(n1, n2)| {
        v1.push(n1);
        v2.push(n2);
    });
    let sum = v1
        .iter()
        .map(|n1| {
            let occurances = v2.iter().filter(|n2| n1 == *n2).count();
            *n1 * occurances as i32
        })
        .sum();
    Ok(sum)
}
#[test]
fn test_part1() {
    let test_in = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let res = part1(test_in).unwrap();
    assert_eq!(res, 11)
}
#[test]
fn test_part2() {
    let test_in = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let res = part2(test_in).unwrap();
    assert_eq!(res, 31)
}
