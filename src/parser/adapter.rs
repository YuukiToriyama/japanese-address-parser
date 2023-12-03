use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn adapt_variety_of_spelling(
    input: &str,
    region_name: &String,
    variety_of_spelling: Vec<&str>,
) -> Option<(String, String)> {
    if variety_of_spelling
        .iter()
        .all(|s| region_name.contains(s) == false)
    {
        return None;
    }
    for permutation in variety_of_spelling.iter().permutations(2) {
        if region_name.contains(permutation[0]) {
            let edited_region_name = region_name.replace(permutation[0], permutation[1]);
            if let Ok((rest, _)) =
                tag::<&str, &str, VerboseError<&str>>(&edited_region_name).parse(input)
            {
                return Some((rest.to_string(), region_name.to_string()));
            };
        }
    }
    None
}
