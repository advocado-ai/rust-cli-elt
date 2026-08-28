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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    /*
    todo: 
    For a further improvement, return an iterator from the search function by removing the call to collect and changing the return type to impl Iterator<Item = &'a str> so that the function becomes an iterator adapter. Note that you’ll also need to update the tests! Search through a large file using your minigrep tool before and after making this change to observe the difference in behavior. Before this change, the program won’t print any results until it has collected all of the results, but after the change, the results will be printed as each matching line is found because the for loop in the run function is able to take advantage of the laziness of the iterator.    
     */

    //unimplemented!();
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
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


        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!{
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        };
    }

}