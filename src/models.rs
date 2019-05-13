
#[derive(Debug,Queryable)]
pub struct Word {
    pub word: String,
    pub def: String,
    pub id: i32,
    pub pos: String,
    pub origin: String,
}
