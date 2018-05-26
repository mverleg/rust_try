Borrow checker problems for parser that can delegate 
===============================

I have several parsers. There is a top-level one that can delegate to another one.

`Parser`s get their input from a `Reader` (mutable). I want only one `Parser` to be able to parse at a time, by only one parser should have the `Reader`.

I did this by making an enum for the top level parser that is either the reader, or the delegated parser (which has a reader). That way it can only read when not delegated, which is what I want.

From the top level parser, I need to mutably borrow this enum to determine what to do and to get the reader or delegate parser. The problem is that if I want to start or stop delegating, I need to move the `Reader` around. But it's still mutably borrowed at this point.

I have created a minimal example, but it is still rather long:
 
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

The first errors are (further ones are similar for other lines):

    error[E0507]: cannot move out of borrowed content
      --> src/main.rs:45:51
       |
    45 |                         self.reader_or_delegate = self.reader_or_delegate.as_delegate();
       |                                                   ^^^^ cannot move out of borrowed content
    
    error[E0499]: cannot borrow `*self` as mutable more than once at a time
      --> src/main.rs:46:25
       |
    42 |             ReaderOrDelegate::Delegate(ref mut delegate) => {
       |                                        ---------------- first mutable borrow occurs here
    ...
    46 |                         self.parse()
       |                         ^^^^ second mutable borrow occurs here
    ...
    60 |         }
       |         - first borrow ends here

I believe I can probably solve this by taking `reader` out of `ReaderOrDelegate` and putting it in each parser as an `Rc<RefCell<Parser>>>`. But I think having it in the enum is more logical: there should be only one parser at a time able to use the reader. Is this possible?

