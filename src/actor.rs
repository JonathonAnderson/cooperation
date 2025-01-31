use std::collections::{HashMap, VecDeque};

struct Actor {
  identity: u32,
  mood: Mood,
  strategy: HashMap<VecDeque<Interaction>, Choice>,
  memory: HashMap<u32, VecDeque<Interaction>>,
}

enum Mood {
  Content,
  Provoked,
  Contrite,
}

enum Result {
  Cooperate,
  Traitor,
  Betrayed,
  DoubleCross,
}

enum Choice {
  Cooperate,
  Defect
}

struct Interaction {
  choice: Choice,
  result: Result,
}