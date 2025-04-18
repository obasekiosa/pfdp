fn comformity(line: &[bool]) -> (usize, String) {
    let (mut f, mut t, mut index, mut prev) = (0usize, 0usize, 0usize, 0usize);
    let (mut f_intervals, mut t_intervals) = (String::new(), String::new());

    while index < line.len() {
        if index == 0 || line[index] != line[index - 1] {
            match line[index] {
                true => {
                    t += 1;
                    if index != 0 {
                        f_intervals.push_str(&format!("{prev}-{},", index - 1));
                    }
                }
                false => {
                    f += 1;
                    if index != 0 {
                        t_intervals.push_str(&format!("{prev}-{},", index - 1));
                    }
                }
            }

            prev = index;
        }

        if index == line.len() - 1 {
            match line[index] {
                true => t_intervals.push_str(&format!("{prev}-{},", index)),
                false => f_intervals.push_str(&format!("{prev}-{},", index)),
            }
        }

        index += 1;
    }

    if f < t {
        (f, f_intervals)
    } else {
        (t, t_intervals)
    }
}

fn comformity2(line: &[bool]) -> (usize, String) {
    if line.len() < 2 {
        return (0usize, String::new());
    }

    let non_conformers = !line[0];

    let (mut index, mut prev, mut count, mut intervals) = (1usize, 0usize, 0usize, String::new());

    while index < line.len() {
        if line[index] != line[index - 1] && line[index] == non_conformers {
            prev = index;
        } else if line[index] != line[index - 1] {
            count += 1;
            intervals.push_str(&format!("{prev}-{},", index - 1));
        }

        if index == line.len() - 1 && line[index] == non_conformers {
            count += 1;
            intervals.push_str(&format!("{prev}-{},", index))
        }

        index += 1;
    }

    (count, intervals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comformity() {
        let line = [
            false, true, true, false, true, false, false, true, true, true, false,
        ];
        let (ans, commands) = comformity(&line);
        assert_eq!(ans, 3);
        println!(
            "you need {} commands to make all of {:?} conform",
            ans, line
        );
        println!("these are the commands {}", commands)
    }

    #[test]
    fn test_comformity2() {
        let line = [
            false, true, true, false, true, false, false, true, true, true, false,
        ];
        let (ans, commands) = comformity2(&line);
        assert_eq!(ans, 3);
        println!(
            "you need {} commands to make all of {:?} conform",
            ans, line
        );
        println!("these are the commands {}", commands)
    }
}
