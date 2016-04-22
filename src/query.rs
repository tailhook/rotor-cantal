use std::time::Instant;
use std::collections::HashMap;

use rotor::Time;
use Key;

pub type TimeStamp = u64;  // Milliseconds
pub type TimeSlice = (TimeStamp, TimeStamp);

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum Value {
    Counter(u64),
    Integer(i64),
    Float(f64),
    State((u64, String)),
}

probor_enum_encoder_decoder!(Value {
    #0 State(item #1),
    #1 Counter(value #1),
    #2 Integer(value #1),
    #3 Float(value #1),
});

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Signed,
    Unsigned,
    Float,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Counter(u8),
    Level(u8, LevelType),
    State(u16),
    Pad(u16),
    Unknown(u16),
}

#[derive(Debug)]
pub enum Conflict {
    CantSumChart,
    Dissimilar,
    CantSumTimestamps,
    CantSumStates,
    CantDerive,
}

probor_enum_encoder_decoder!(Conflict {
    #100 CantSumChart(),
    #101 Dissimilar(),
    #102 CantSumTimestamps(),
    #103 CantSumStates(),
    #104 CantDerive(),
});

#[derive(Debug)]
pub enum Dataset {
    SingleSeries(Key, Chunk, Vec<TimeStamp>),
    MultiSeries(Vec<(Key, Chunk, Vec<TimeStamp>)>),
    SingleTip(Key, Value, TimeSlice),
    MultiTip(Vec<(Key, Value, TimeSlice)>),
    Chart(HashMap<String, usize>),
    // TODO(tailhook) multi-chart
    Empty,
    Incompatible(Conflict),
}

// Keep in sync with query::rule::Expectation
probor_enum_encoder_decoder!(Dataset {
    #100 SingleSeries(key #1, data #2, timestamps #3),
    #101 MultiSeries(data #1),
    #200 SingleTip(key #1, value #2, tslice #3),
    #201 MultiTip(data #1),
    #300 Chart(data #1),
    #998 Empty(),
    #999 Incompatible(reason #1),
});

#[derive(Debug, Clone)]
pub enum Chunk {
    State((TimeStamp, String)),
    Counter(Vec<Option<u64>>),
    Integer(Vec<Option<i64>>),
    Float(Vec<Option<f64>>),
}

probor_enum_encoder_decoder!(Chunk {
    #0 State(pair #1),
    #1 Counter(items #1),
    #2 Integer(items #1),
    #3 Float(items #1),
});

#[derive(Debug)]
pub struct RemoteQuery {
    pub timestamp: Instant,
    pub received: Time,
    pub items: HashMap<String, Dataset>,
}

