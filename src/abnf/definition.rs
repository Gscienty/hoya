#[derive(Clone, Copy)]
pub enum RepeatTimes {
    Times(i64),
    Infinity,
}

#[derive(Clone)]
pub enum BnfDefinition {
    Series(Vec<Box<BnfDefinition>>),
    Terminal(String),
    Rule(String),
    Group(Box<BnfDefinition>),
    Choose(Vec<BnfDefinition>),
    Options(Box<BnfDefinition>),
    Range((i64, i64)),
    Repeat((RepeatTimes, RepeatTimes, Box<BnfDefinition>)),
}
