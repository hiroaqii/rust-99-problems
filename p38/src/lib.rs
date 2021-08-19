/// Compare the two methods of calculating Euler's totient function.
/// Use the solutions of problems P34 and P37 to compare the algorithms. Take the number of logical inferences as a measure for efficiency. Try to calculate phi(10090) as an example.

#[cfg(test)]
mod tests {
    #[test]
    fn test(){
        assert_eq!(p34::totient_phi(10090), p37::phi(10090));
    }
}
