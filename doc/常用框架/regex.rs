// 
// regex: 正则表达式处理
//      regex::Regex
//          pub fn new(re: &str) -> Result<Regex, Error>
//          pub fn is_match(&self, text: &str) -> bool
//          pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>>
//          pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't>
//          pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>>
//          pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't>
//          pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str>
//          pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str>
//          
