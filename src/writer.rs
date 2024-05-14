use std::io::Write;

use crate::{GCodeError, GCodePosition};

pub struct GCodeWriter<'a> {
    writer: Box<dyn Write + 'a>
}

impl<'a> GCodeWriter<'a> {
    pub fn new(writer: impl Write + 'a) -> Result<Self, GCodeError> {
        Ok(Self {
            writer: Box::new(writer)
        })
    }

    pub fn move_to(&mut self, pos: GCodePosition, fast: bool) -> Result<(), GCodeError> {
        let code = if fast { "G00" } else { "G01" };
        let (x, y, z) = pos.as_f64();
        write!(self.writer, "{}", code)?;
        if let Some(val) = x {
            write!(self.writer, " X{:.4}", val)?;
        }
        if let Some(val) = y {
            write!(self.writer, " Y{:.4}", val)?;
        }
        if let Some(val) = z {
            write!(self.writer, " Z{:.4}", val)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), GCodeError> {
        if let Err(_) = self.writer.flush() {
            Err(GCodeError::IOError)
        } else {
            Ok(())
        }
    }

    /// Drops self and returns the contained writer
    pub fn writer(self) -> Box<dyn Write + 'a> {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn move_to() -> Result<(), GCodeError> {
        fn test(pos: GCodePosition, res: &str) -> Result<(), GCodeError> {
            let mut data = vec!();
            let bw = BufWriter::new(&mut data);
            let mut gcw = GCodeWriter::new(bw)?;

            gcw.move_to(pos, false)?;
            gcw.writer();

            assert_eq!(String::from_utf8_lossy(&data), res);
            Ok(())
        }

        test(GCodePosition::from_f64_full(1.0, 2.0, 3.0)?, "G01 X1.0000 Y2.0000 Z3.0000")?;
        test(GCodePosition::from_f64_full(1.1, 2.2, 3.3)?, "G01 X1.1000 Y2.2000 Z3.3000")?;
        test(GCodePosition::from_f64(Some(1.0), None, Some(3.0))?, "G01 X1.0000 Z3.0000")?;

        Ok(())
    }
}
