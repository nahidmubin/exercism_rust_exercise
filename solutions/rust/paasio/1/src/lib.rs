use std::{io::{Read, Result, Write}};

pub struct ReadStats<R: Read>{
    wrapped_object: R,
    bytes_read: usize,
    operations: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {wrapped_object: wrapped, bytes_read: 0, operations: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped_object
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let res = self.wrapped_object.read(buf);
        if let Ok(val) = res {
            self.bytes_read += val;
            self.operations += 1;
        }

        res
    }
}

pub struct WriteStats<W: Write>{
    wrapped_object: W,
    bytes_read: usize,
    operations: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {wrapped_object: wrapped, bytes_read: 0, operations: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped_object
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = self.wrapped_object.write(buf);
        if let Ok(val) = res {
            self.bytes_read += val;
            self.operations += 1;
        }

        res
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped_object.flush()
    }
}
