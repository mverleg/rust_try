#![feature(nll)]
use std::cell::RefCell;
use std::mem::replace;
use std::rc::Rc;

struct Reader_ {
    i: u8,
}
impl Reader_ {
    fn next(&mut self) -> u8 {
        /* some logic here */
        self.i += 1;
        self.i
    }
}
type Reader = Rc<RefCell<Reader_>>;

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
                let del_res = delegate.parse();
                match del_res {
                    0 => {
                        let reader = delegate.reader.clone();
                        self.reader_or_delegate = ReaderOrDelegate::Read(reader);
                        self.parse()
                    },
                    x => 2 * x
                }
            },
            ReaderOrDelegate::Read(ref mut reader) => {
                match reader.borrow_mut().next() {
                    0 => {
                        let subparser = AnotherParser { reader: reader.clone() };
                        replace(&mut self.reader_or_delegate, ReaderOrDelegate::Delegate(subparser));
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

impl Parser for AnotherParser {
    fn parse(&mut self) -> u8 {
        self.reader.borrow_mut().next() * 2
    }
}

fn main() {
    let mut parser = OneParser { reader_or_delegate: ReaderOrDelegate::Read(Rc::new(RefCell::new(Reader_{ i: 7 }))) };
    parser.parse();
}
