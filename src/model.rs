use chrono::{DateTime, Local};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SetType {
    WarmUp,
    Working,
    Drop,
    Failure,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Exercise {
    name: String,
    weight: u16,
    reps: u8,
    set_type: SetType,
    reps_in_reserve: Option<u8>,
    notes: Option<String>,
    min_goal: Option<u8>,
    max_goal: Option<u8>,
    completed: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Workout {
    name: Option<String>,
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
    exercises: Vec<Vec<Exercise>>,
    current_exercise: (usize, usize),
}

#[allow(dead_code)]
#[derive(Debug)]
struct ExerciseTemplate {
    name: String,
    min_reps: u8,
    max_reps: u8,
    sets: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Routine {
    name: String,
    // exercise, min reps, max reps
    exercise_and_goals: Vec<ExerciseTemplate>,
}

#[allow(dead_code)]
impl Routine {
    fn new(name: String) -> Self {
        Routine {
            name: name.clone(),
            exercise_and_goals: Vec::new(),
        }
    }

    fn add_exercise(&mut self, name: String, min_reps: u8, max_reps: u8, sets: u8) {
        let template = ExerciseTemplate {
            name,
            min_reps,
            max_reps,
            sets,
        };

        self.exercise_and_goals.push(template);
    }
}

#[allow(dead_code)]
impl Exercise {
    fn new(
        name: String,
        weight: u16,
        reps: u8,
        set_type: SetType,
        rir: Option<u8>,
        notes: Option<String>,
        min_goal: Option<u8>,
        max_goal: Option<u8>,
        completed: bool,
    ) -> Self {
        Exercise {
            name: name.clone(),
            weight,
            reps,
            set_type,
            reps_in_reserve: rir,
            notes,
            min_goal,
            max_goal,
            completed,
        }
    }
}

#[allow(dead_code)]
impl Workout {
    fn new() -> Self {
        Workout {
            name: None,
            start: chrono::Local::now(),
            end: None,
            exercises: Vec::new(),
            current_exercise: (0, 0),
        }
    }
    fn from_template(routine: Routine) -> Self {
        let mut exercises = Vec::new();

        for exercise_and_goal in routine.exercise_and_goals.iter() {
            let mut set = Vec::with_capacity(exercise_and_goal.sets as usize + 5);
            for _ in 0..exercise_and_goal.sets {
                set.push(Exercise {
                    name: exercise_and_goal.name.clone(),
                    weight: 0,
                    reps: 0,
                    set_type: SetType::Working,
                    reps_in_reserve: None,
                    notes: None,
                    min_goal: Some(exercise_and_goal.min_reps),
                    max_goal: Some(exercise_and_goal.max_reps),
                    completed: false,
                })
            }
            exercises.push(set);
        }
        Workout {
            name: Some(routine.name),
            start: chrono::Local::now(),
            end: None,
            exercises,
            current_exercise: (0, 0),
        }
    }
    pub fn end_workout(&mut self) {
        self.end = Some(chrono::Local::now())
    }

    // gets a particular exercise based on the name and set number
    fn get_set(exercise: usize, set_number: usize) -> Exercise {}

    // gets a particular exercise based on the name and set number and updates it
    fn update_set(exercise: usize, set_number: usize) {}

    fn remove_set(exercise: usize, set_number: usize) {}

    // adds a set to an exercise
    fn add_set(exercise: usize, set_number: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    mod workout_tests {
        use super::*;

        #[test]
        fn end_workout() {
            let mut workout = Workout::new();

            if let Some(_value) = workout.end {
                assert!(false);
            } else {
                assert!(true);
            }

            workout.end_workout();
            if let Some(_value) = workout.end {
                assert!(true);
            } else {
                assert!(false);
            }
        }

        #[test]
        fn new_workout_from_routine() {
            let mut routine = Routine::new("push_1".to_string());
            routine.add_exercise("Bench Press".to_string(), 5, 8, 3);
            routine.add_exercise("Incline Smith Press".to_string(), 5, 8, 3);
            routine.add_exercise("Cable Flys".to_string(), 12, 20, 3);
            routine.add_exercise("Lateral Raise".to_string(), 12, 20, 3);

            let workout = Workout::from_template(routine);

            let ex = Exercise::new(
                "Bench Press".to_string(),
                0,
                0,
                SetType::Working,
                None,
                None,
                Some(5),
                Some(8),
                false,
            );

            dbg!();

            assert_eq!(ex, workout.exercises[0][0])
        }
    }

    mod routine_tests {
        use super::Routine;

        #[test]
        fn new_routine() {
            let mut routine = Routine::new("push_1".to_string());
            routine.add_exercise("Bench Press".to_string(), 5, 8, 3);
            routine.add_exercise("Incline Smith Press".to_string(), 5, 8, 3);
            routine.add_exercise("Cable Flys".to_string(), 12, 20, 3);
            routine.add_exercise("Lateral Raise".to_string(), 12, 20, 3);
            println!("{:?}", routine);

            assert_eq!(routine.name, "push_1".to_string());
        }
    }
}
