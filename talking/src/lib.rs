pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_question = text.trim_end().ends_with('?');
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());

    if is_question && is_yelling {
       return  "Quiet, I am thinking!";
    } else if is_yelling {
      return   "There is no need to yell, calm down!";
    } else if is_question {
       return  "Sure.";
    } else {
      return   "Interesting";
    }
}
