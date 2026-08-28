/* 
TDD:
1.Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!
 */

/*
lifetime parameter specifies which argument lifetime is connected to the lifetime of the return value, ie. vector contains string slices that reference slices of the argument contents(rather than the query), 
"data returned b search function lives as long as the data passed into the search function in the contents argument"
 */
pub fn search<'a>(query: &'a str, content: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>{
    /*
    Box<dyn Iterator<Item = &'a str> + 'a> the Iterator needs a lifetime param equal to the strings being iterated ie. default 'static would outlive the strings 'a so it might try to iterate on non-existent str's
     */

    //unimplemented!();
    Box::new(content
        .lines()
        .filter( move |line| line.contains(query)))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a>{

    let query = query.to_lowercase();
    
    Box::new(content
        .lines()
        .filter(move |line|line.to_lowercase().contains(&query)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";


        assert_eq!("safe, fast, productive.", search(query, contents)
            .next()
            .expect("test search failed"));
    }
 
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results: Vec<&str> = search_case_insensitive(query, content).collect();


        assert_eq!(
            vec!["Rust:", "Trust me."],
            results);
    }


}