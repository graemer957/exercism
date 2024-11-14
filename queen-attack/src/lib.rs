#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    file: i32,
    rank: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    const MIN: i32 = 0;
    const MAX: i32 = 7;

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < ChessPosition::MIN
            || file < ChessPosition::MIN
            || rank > ChessPosition::MAX
            || file > ChessPosition::MAX
        {
            None
        } else {
            Some(ChessPosition { file, rank })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.file == other.position.file
            || self.position.rank == other.position.rank
            || (self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs()
    }
}
