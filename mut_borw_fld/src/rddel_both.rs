
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
impl ReaderOrDelegate {
    fn as_read(self) -> Self {
        match self {
            ReaderOrDelegate::Delegate(delegate) => ReaderOrDelegate::Read(delegate.consume()),
            reader => reader
        }
    }
    fn as_delegate(self) -> Self {
        match self {
            ReaderOrDelegate::Read(reader) => ReaderOrDelegate::Delegate(AnotherParser { reader }),
            delegate => delegate
        }
    }
}

struct OneParser {
    reader_or_delegate: ReaderOrDelegate,
    // This parser either parses by itself, or delegates to another parser
}
impl Parser for OneParser {
    fn parse(&mut self) -> u8 {
        match self.reader_or_delegate {
            ReaderOrDelegate::Delegate(ref mut delegate) => {
                match delegate.parse() {
                    0 => {
                        self.reader_or_delegate = self.reader_or_delegate.as_delegate();
                        self.parse()
                    },
                    x => 2 * x
                }
            },
            ReaderOrDelegate::Read(ref mut reader) => {
                match reader.next() {
                    0 => {
                        self.reader_or_delegate = self.reader_or_delegate.as_read();
                        self.parse()
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
    fn consume(self) -> Reader {
        self.reader
    }
}
impl Parser for AnotherParser {
    fn parse(&mut self) -> u8 {
        self.reader.next() * 2
    }
}

fn main() {}
