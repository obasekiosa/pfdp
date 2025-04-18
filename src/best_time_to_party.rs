fn best_time_to_party(schedule: &Vec<[u32; 2]>) {
    let mut start = schedule[0][0]; // min in time
    let mut end = start; // max out time

    use std::cmp;
    for &[s, e] in schedule {
        start = cmp::min(start, s);
        end = cmp::max(end, e);
    }

    let (mut best_time, mut best_count) = (start, 0);

    for time in start..(end + 1) {
        let mut count = 0;
        for &[s, e] in schedule {
            if time >= s && time < e {
                count += 1;
            }
        }
        (best_time, best_count) = if best_count <= count {
            (time, count)
        } else {
            (best_time, best_count)
        };
    }
    println!(
        "{:?} come at {best_time} you'd meet {best_count} clebrities",
        schedule
    );
}

fn best_time_to_party2(schedule: &Vec<[u32; 2]>) {
    let mut buff = Vec::<(u32, bool)>::with_capacity(schedule.len());

    for &[s, e] in schedule.iter() {
        buff.push((s, true));
        buff.push((e, false));
    }

    buff.sort();
    let mut rsum = 0;
    let mut best: Option<u32> = None;
    let mut best_sum = 0;

    for &(time, status) in buff.iter() {
        if status {
            rsum += 1;
        } else {
            rsum -= 1;
        }
        best_sum = if rsum > best_sum {
            best = Some(time);
            rsum
        } else {
            best_sum
        };
    }

    println!(
        "{:?} come at {} you'd meet {} clebrities",
        schedule,
        best.unwrap_or(0),
        best_sum
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_time_to_party() {
        let mut schedule: Vec<[u32; 2]> =
            vec![[6, 8], [6, 7], [7, 9], [10, 11], [10, 12], [8, 10], [9, 11]];
        best_time_to_party(&mut schedule);
    }

    #[test]
    fn test_best_time_to_party2() {
        let mut schedule: Vec<[u32; 2]> =
            vec![[6, 8], [6, 7], [7, 9], [10, 11], [10, 12], [8, 10], [9, 11]];
        best_time_to_party2(&mut schedule);
    }
}
