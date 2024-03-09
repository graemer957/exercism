#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

// Not wanting to touch `Bucket` given to us above
struct Container {
    capacity: u8,
    volume: u8,
    label: Bucket,
}

impl Container {
    const fn new(capacity: u8, bucket: Bucket) -> Self {
        Self {
            capacity,
            volume: 0,
            label: bucket,
        }
    }

    fn empty(&mut self) {
        self.volume = 0;
    }

    fn is_empty(&self) -> bool {
        self.volume == 0
    }

    fn fill(&mut self) {
        self.volume = self.capacity;
    }

    fn is_full(&self) -> bool {
        self.volume == self.capacity
    }

    fn pour(&mut self, other: &mut Self) {
        let max_transfer = std::cmp::min(other.capacity - other.volume, self.volume);
        other.volume += max_transfer;
        self.volume -= max_transfer;
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // It it not possible to solve if the `difference` does not divide equally into the goal
    // unless the goal is the capacity of either container, in which case it is a simple fill
    let difference = i16::from(capacity_1) - i16::from(capacity_2);
    if capacity_1 != goal && capacity_2 != goal && difference % i16::from(goal) != 0 {
        return None;
    }

    // Assume starting bucket is `One`
    let mut start_container = Container::new(capacity_1, Bucket::One);
    let mut other_container = Container::new(capacity_2, Bucket::Two);

    // ...otherwise swap them over
    if start_bucket != &Bucket::One {
        std::mem::swap(&mut start_container, &mut other_container);
    }

    let mut moves = 0;
    while start_container.volume != goal && other_container.volume != goal {
        if start_container.is_empty() {
            start_container.fill();
        } else if other_container.capacity == goal {
            other_container.fill();
        } else if other_container.is_full() {
            other_container.empty();
        } else {
            start_container.pour(&mut other_container);
        }

        moves += 1;
    }

    // Determine which container hit the goal
    let (filled_container, other_container) = if start_container.volume == goal {
        (start_container, other_container)
    } else {
        (other_container, start_container)
    };

    Some(BucketStats {
        moves,
        goal_bucket: filled_container.label,
        other_bucket: other_container.volume,
    })
}
