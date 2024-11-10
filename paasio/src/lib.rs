use std::io::{Read, Result, Write};

pub type ReadStats<T> = IoStatistics<T>;
pub type WriteStats<T> = IoStatistics<T>;

pub struct IoStatistics<T> {
    io: T,
    ops: usize,
    bytes: usize,
}

impl<T> IoStatistics<T> {
    pub fn new(io: T) -> IoStatistics<T> {
        IoStatistics {
            io,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.io
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    fn process(&mut self, result: Result<usize>) -> Result<usize> {
        if let Ok(bytes) = result {
            self.ops += 1;
            self.bytes += bytes;
        }
        result
    }
}

impl<T: Read> IoStatistics<T> {
    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<T: Write> IoStatistics<T> {
    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for IoStatistics<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.io.read(buf);
        self.process(result)
    }
}

impl<W: Write> Write for IoStatistics<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.io.write(buf);
        self.process(result)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}
