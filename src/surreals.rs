use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::zip;
use std::sync::Mutex;
use lazy_static::lazy_static;

const ZERO: RawSurreal = RawSurreal { left: None, right: None };

#[derive(Debug)]
pub struct RawSurreal {
    left: Option<usize>,
    right: Option<usize>,
}

impl RawSurreal {
    pub fn new(left: Option<usize>, right: Option<usize>) -> RawSurreal {
        RawSurreal { left, right }
    }
    pub fn less_than(&self, other: &RawSurreal, list: &Vec<RawSurreal>) -> bool {
        (self.left.is_none() || (self.left.is_some() && !other.less_than( &list[self.left.unwrap()], list )) )
        && (other.right.is_none() || (other.right.is_some() && list[other.right.unwrap()].less_than(self, list)) )
    }
    pub fn equal(&self, other: &RawSurreal, list: &Vec<RawSurreal>) -> bool {
        self.less_than(other, list) && other.less_than(self, list)
    }
}

pub struct SurrealNumbers {
    numbers_line: VecDeque<usize>, // Vector of indexes
    numbers_by_day: Vec<RawSurreal>,
    day: usize,
}

impl SurrealNumbers {
    pub fn new() -> SurrealNumbers {
        SurrealNumbers { numbers_line: VecDeque::from([0]), numbers_by_day: vec![ZERO], day: 0 }
    }
    pub fn generate_next_day(&mut self) {
        let base_length = self.numbers_by_day.len();

        // First, generate all new numbers
        self.numbers_line.make_contiguous();
        let mut new_numbers: Vec<_> = self.numbers_line.as_slices().0.windows(2)
            .into_iter().map(|arr| RawSurreal::new(Some(arr[0]), Some(arr[1]))).collect();
        // Add the ending numbers
        new_numbers.push(RawSurreal::new(Some(*self.numbers_line.back().unwrap()), None));
        new_numbers.push(RawSurreal::new(None, Some(self.numbers_line[0])));
        self.numbers_by_day.append(&mut new_numbers);

        // Update the number line
        self.numbers_line = self.numbers_line.iter()
            .zip(base_length..)
            .map(|tup| [*tup.0, tup.1]) // There's gotta be a better way to do this
            .flatten()
            .collect();
        self.numbers_line.push_front(self.numbers_by_day.len() - 1);
    }
    fn index(&self, index: usize) -> &RawSurreal {
        &self.numbers_by_day[index]
    }
    pub fn numbers(&self) -> &Vec<RawSurreal> {
        &self.numbers_by_day
    }
    pub fn number_line(&self) -> Vec<&RawSurreal> {
        self.numbers_line.iter().map(|idx| &self.numbers_by_day[*idx]).collect()
    }
}

lazy_static! {
    pub static ref SURREALS: Mutex<SurrealNumbers> = Mutex::new( SurrealNumbers::new() );
}

#[derive(Copy, Clone, Eq, PartialEq)] // Should be alright to derive equality, since we won't have any duplicates
struct Surreal {
    index: usize
}

impl Surreal {
    pub fn new(left: Option<Surreal>, right: Option<Surreal>) -> Surreal {
        let raw = RawSurreal::new(left.map(|s| s.index), right.map(|s| s.index));
        let mut unlocked_surreals = SURREALS.lock().unwrap();
        // find equivalent
        let mut found_idx = 0;
        let mut found = false;
        for (idx, num) in unlocked_surreals.numbers_by_day.iter().enumerate() {
            found_idx = idx;
            if raw.equal(num, &unlocked_surreals.numbers_by_day) {
                found = true;
                break;
            }
        }
        if !found {
            // This should only be possible if we have a number from the most recent day
            // In this case, make a new day, then check again
            unlocked_surreals.generate_next_day();
            for (idx, num) in unlocked_surreals.numbers_by_day[found_idx + 1..].iter().enumerate() {
                found_idx = idx;
                if raw.equal(num, &unlocked_surreals.numbers_by_day) {
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("Wasn't able to find surreal")
            }
        }
        let index = found_idx as usize;
        Surreal { index }
    }

    fn index(&self) -> usize { self.index }
}

impl PartialOrd for Surreal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let surreals = SURREALS.lock().unwrap();
        let self_raw = &surreals.numbers_by_day[self.index];
        let other_raw = &surreals.numbers_by_day[other.index];

        let mut le = self_raw.less_than(other_raw, &surreals.numbers_by_day);
        let mut ge = other_raw.less_than(self_raw, &surreals.numbers_by_day);

        Some (
            if le & ge { Ordering::Equal }
            else if le { Ordering::Less }
            else { Ordering::Greater }
        )
    }
}


impl Ord for Surreal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}