struct User {
    id: u64,
    nickname: String,
}

struct Problem {
    id: u64,
    title: String,
    description: String,
}

impl Problem {
    fn display(&self) {
        println!("Задача №{}: \"{}\"\n", self.id, self.title);
    }
}

// ProgrammingLanguage = C++, Pascal, Python, Rust;
enum ProgrammingLanguage {
    Cxx,
    Pascal,
    Python,
    Rust,
}

struct Solution {
    id: u64,
    problem_id: u64,
    contest_id: u64,
    author_id: u64,
    programming_language: ProgrammingLanguage,
    source_code: String,
}

struct Contest {
    id: u64,
    title: String,
    problem_ids: Vec<u64>,
}

impl Contest {
    fn display(&self) {
        println!("Турнир #{}: \"{}\"", self.id, self.title);
        for problem_id in &self.problem_ids {
            println!("Задача с ID #{} есть в турнире", problem_id);
        }
        println!();
    }

    fn add_problem(&mut self, problem: &Problem) {
        self.problem_ids.push(problem.id.clone());
    }
}

fn main() {
    let user_root = User {
        id: 1000,
        nickname: "root".to_string(),
    };

    // user_root.display();

    let user_frol = User {
        id: 1001,
        nickname: "frol".to_string(),
    };

    let problem_a_plus_b = Problem {
        id: 1000,
        title: "A + B".to_string(),
        description: "...".to_string(),
    };

    problem_a_plus_b.display();

    let problem_a_minus_b = Problem {
        id: 1001,
        title: "A - B".to_string(),
        description: "...".to_string(),
    };

    let first_contest_problems = [problem_a_plus_b, problem_a_minus_b];
    for problem in &first_contest_problems {
        problem.display();
    }

    let mut first_contest = Contest {
        id: 1,
        title: "Первый турнир".to_string(),
        problem_ids: first_contest_problems
            .iter()
            .map(|problem| problem.id)
            .collect(),
    };

    first_contest.display();

    let mut problem_a_divide_b = Problem {
        id: 1002,
        title: "A / B".to_string(),
        description: "...".to_string(),
    };

    first_contest.add_problem(&problem_a_divide_b);

    first_contest.display();

    let mut contest_8b = Contest {
        id: 2,
        title: "Турнир 8-Б".to_string(),
        problem_ids: vec![],
    };
    contest_8b.display();

    contest_8b.add_problem(&problem_a_divide_b);

    contest_8b.display();

    problem_a_divide_b.id = 666;
    problem_a_divide_b.display();

    contest_8b.display();

    let solution_1 = Solution {
        id: 1,
        problem_id: problem_a_divide_b.id,
        contest_id: contest_8b.id,
        author_id: user_frol.id,
        programming_language: ProgrammingLanguage::Rust,
        source_code: "fn main() {}".to_string(),
    };

    // solution_1.display();

    println!("Bye.");
}
