use std::collections::{HashSet, VecDeque};

trait State: std::hash::Hash + Eq + std::fmt::Debug + Clone {}
impl<T: std::hash::Hash + Eq + std::fmt::Debug + Clone> State for T {}

trait Symbol: std::hash::Hash + Eq + std::fmt::Debug + Clone {}
impl<T: std::hash::Hash + Eq + std::fmt::Debug + Clone> Symbol for T {}

type Transition<M, T> = Box<dyn Fn(M, T) -> (M, Movement, T)>;

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

/// Hopcroft & Ullman's Turing machine
struct TuringMachine<M: Symbol, T: State> {
    // Q
    state_set: HashSet<T>,
    // Г
    symbol_set: HashSet<M>,
    // b
    blank_symbol: M,
    // Σ
    // set of initial symbols
    // δ
    transition: Transition<M, T>,
    // q0
    initial_state: T,
    // F
    final_state_set: HashSet<T>,

    // implementation specific fields
    head: usize,
    tape: VecDeque<M>,
    current_state: T,
}

impl<M: Symbol, T: State> TuringMachine<M, T> {
    fn new(
        state_set: HashSet<T>,
        symbol_set: HashSet<M>,
        blank_symbol: M,
        initial_state: T,
        final_state_set: HashSet<T>,
        transition: Transition<M, T>,
        tape: VecDeque<M>,
    ) -> Self {
        assert!(!state_set.is_empty());
        assert!(state_set.contains(&initial_state));

        assert!(!symbol_set.is_empty());
        assert!(symbol_set.contains(&blank_symbol));

        let head = Default::default();
        let current_state = initial_state.clone();

        Self {
            state_set,
            symbol_set,
            blank_symbol,
            initial_state,
            final_state_set,
            transition,
            head,
            tape,
            current_state,
        }
    }

    fn step(&mut self) -> bool {
        if self.final_state_set.contains(&self.current_state) {
            return false;
        }
        let (symbol, movement, state) =
            (self.transition)(self.tape[self.head].clone(), self.current_state.clone());

        println!(
            "{:?} {:?} => {:?} {:?} {:?}",
            self.current_state, self.tape[self.head], state, symbol, movement
        );
        self.current_state = state;
        self.tape[self.head] = symbol;

        match movement {
            Movement::Left => {
                if self.head == 0 {
                    self.tape.push_front(self.blank_symbol.clone());
                } else {
                    self.head -= 1;
                }
            }
            Movement::Right => {
                self.head += 1;

                if self.tape.len() <= self.head {
                    self.tape.push_back(self.blank_symbol.clone());
                }
            }
        }
        true
    }

    fn run(&mut self) -> VecDeque<M> {
        while self.step() {}
        while !self.final_state_set.contains(&self.current_state) {
            self.step();
        }
        self.tape.clone()
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, VecDeque};

    use crate::{Movement, TuringMachine};

    #[test]
    fn busy_beaver() {
        let transition = |symbol: char, current_state: char| match (symbol, current_state) {
            ('0', 'A') => ('1', Movement::Right, 'B'),
            ('0', 'B') => ('1', Movement::Left, 'A'),
            ('0', 'C') => ('1', Movement::Left, 'B'),
            ('1', 'A') => ('1', Movement::Left, 'C'),
            ('1', 'B') => ('1', Movement::Right, 'B'),
            ('1', 'C') => ('1', Movement::Right, 'H'),
            _ => unreachable!(),
        };
        let transition = Box::new(transition);

        let mut turing_machine = TuringMachine::new(
            HashSet::from(['A', 'B', 'C', 'H']),
            HashSet::from(['0', '1']),
            '0',
            'A',
            HashSet::from(['H']),
            transition,
            VecDeque::from(['0']),
        );
        let result = turing_machine.run();
        println!("RESULT: {:?}", result);
    }
}
