
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
    Delegate(Box<Parser>),
}

struct OneParser {
    reader_or_delegate: ReaderOrDelegate,
    // This parser either parses by itself, or delegates to another parser
}
impl Parser for OneParser {
    fn parse(&mut self) -> u8 {
        match self.reader_or_delegate {
            ReaderOrDelegate::Delegate(ref mut delegate) => {
                delegate.parse()
            },
            ReaderOrDelegate::Read(ref mut reader) => {
                match reader.next() {
                    0 => {

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
impl Parser for AnotherParser {
    fn parse(&mut self) -> u8 {
        self.reader.next() * 2
    }
}

fn main() {}
