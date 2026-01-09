// source:
// Debasish Ghosh
// https://www.linkedin.com/in/debasishgh/
// https://www.linkedin.com/posts/debasishgh_yet-another-example-of-why-i-like-types-activity-7415070943070453760-vccS/
//
// """
// - expressing “two-ended traversal is available” as a static, checkable capability (DoubleEndedIterator), e.g. is_palindrome(std::io::stdin().lines()) simply won’t compile because `lines` can’t go backwards. That’s not a runtime surprise, it’s a type error.
//
// - keeping it zero-cost (no virtual dispatch unless you opt in) and
//
// - having it compose through all the out-of-the-box iterator adapters so you can build pipelines and still keep that capability.
// """
// KM: Very elegant
//
fn is_palindrome<I>(mut it: I) -> bool 
where
    I: DoubleEndedIterator,
    I::Item: PartialEq,
{
    while let (Some(a), Some(b)) = (it.next(), it.next_back()){
        if a != b {return false;}
    }
    true
}
