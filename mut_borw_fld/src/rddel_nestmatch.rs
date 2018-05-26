#![feature(nll)]
use std::mem::replace;

struct Reader {
    i: u8,
}
impl Reader {
    fn next(&mut self) -> u8 {
        /* some logic here */
        self.i += 1;
        self.i
    }
}

trait Parser {
    fn parse(&mut self) -> u8;
}

enum ReaderOrDelegate {
    Read(Reader),
    Delegate(AnotherParser),  /* Trait object in reality, but keeping it simple here. */
}

struct OneParser {
    reader_or_delegate: ReaderOrDelegate,
}
impl Parser for OneParser {
    fn parse(&mut self) -> u8 {
        match self.reader_or_delegate {
            ReaderOrDelegate::Delegate(ref mut delegate) => {
                match delegate.parse() {
                    0 => {
                        replace(&mut self.reader_or_delegate, ReaderOrDelegate::Read(delegate.consume()));
                        self.parse()
                    },
                    x => 2 * x
                }
            },
            ReaderOrDelegate::Read(ref mut reader) => {
                match reader.next() {
                    0 => {
                        replace(&mut self.reader_or_delegate, ReaderOrDelegate::Delegate(AnotherParser {
                            reader: match self.reader_or_delegate {
                                ReaderOrDelegate::Read(move_reader) => move_reader,
                                _ => panic!(),
                            }
                        }));
                    self.
                    parse()
                },
                x => 3 * x
            }
        },
    }
}
}

struct AnotherParser {
reader: Reader,
}
impl AnotherParser {
fn consume( self ) -> Reader {
self.reader
    }
}
impl Parser for AnotherParser {
    fn parse(&mut self) -> u8 {
        self.reader.next() * 2
    }
}

fn main() {}
