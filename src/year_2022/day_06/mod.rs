use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

struct RingBuffer<const N: usize> {
    buffer: [Option<char>; N],
    insert_index: usize,
}

impl<const N: usize> RingBuffer<N> {
    fn new() -> Self {
        RingBuffer {
            buffer: [None; N],
            insert_index: 0,
        }
    }

    fn push(&mut self, character: char) {
        self.buffer[self.insert_index] = Some(character);
        self.insert_index = (self.insert_index + 1) % N;
    }

    fn all_values_unique(&self) -> bool {
        // return false as long as None values exists
        // the last None will be overwritten last, so only check the last index
        if self.buffer[N - 1].is_none() {
            return false;
        }

        self.buffer.iter().all_unique()
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(6);

    println!(
        "1) First packet marker found at index {}",
        solve_first_part()?
    );
    println!(
        "2) First message marker found at index {}",
        solve_second_part()?
    );

    Ok(())
}

fn solve_first_part() -> MyResult<usize> {
    Ok(find_first_packet_marker(INPUT).unwrap())
}

fn solve_second_part() -> MyResult<usize> {
    Ok(find_first_message_marker(INPUT).unwrap())
}

fn find_first_packet_marker(message: &str) -> Option<usize> {
    let mut ringbuffer = RingBuffer::<4>::new();

    for (index, current_char) in message.chars().enumerate() {
        ringbuffer.push(current_char);

        if ringbuffer.all_values_unique() {
            return Some(index + 1);
        }
    }

    None
}

fn find_first_message_marker(message: &str) -> Option<usize> {
    let mut ringbuffer = RingBuffer::<14>::new();

    for (index, current_char) in message.chars().enumerate() {
        ringbuffer.push(current_char);

        if ringbuffer.all_values_unique() {
            return Some(index + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_packet_marker_examples() {
        assert_eq!(
            find_first_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
        assert_eq!(
            find_first_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
        assert_eq!(
            find_first_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            find_first_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }

    #[test]
    fn find_first_message_marker_examples() {
        assert_eq!(
            find_first_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(19)
        );
        assert_eq!(
            find_first_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(23)
        );
        assert_eq!(
            find_first_message_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(23)
        );
        assert_eq!(
            find_first_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(29)
        );
        assert_eq!(
            find_first_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(26)
        );
    }

    #[test]
    fn find_first_marker_real_input() {
        let result = solve_first_part();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1142);
    }
}
