mod mall;
pub fn biggest_store(mall: mall::Mall) -> mall::Store {}
pub fn highest_paid_employee(mall: mall::Mall) -> Vec<mall::Employee> {}
pub fn nbr_of_employees(mall: mall::Mall) -> usize {}
pub fn check_for_securities(mall: mall::Mall, guards: mall::Guard) {}
pub fn cut_or_raise(mall: mall::Mall) {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
