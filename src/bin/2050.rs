use std::cmp::max;

fn cost(course: i32, time: &Vec<i32>, graph: &Vec<Vec<i32>>, costs: &mut Vec<i32>) -> i32 {
    if costs[course as usize] >= 0 {
        return costs[course as usize];
    }
    let mut course_cost = 0;
    for previous_course in &graph[course as usize] {
        course_cost = max(course_cost, cost(*previous_course, time, graph, costs));
    }
    costs[course as usize] = course_cost + time[course as usize];
    costs[course as usize]
}

pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
    for r in relations {
        graph[r[1] as usize - 1].push(r[0] - 1);
    }
    let mut costs = vec![-1; n as usize];
    let mut max_time = 0;
    for i in 0..n {
        max_time = max(max_time, cost(i, &time, &graph, &mut costs));
    }
    max_time
}

fn main() {}
