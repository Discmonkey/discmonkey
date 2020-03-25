enum LispResult {
    IntType = i32,
    FloatType = i32,
}



trait LispType {

    // hmm
    fn resolve(self) -> Option<LispResult>;
}