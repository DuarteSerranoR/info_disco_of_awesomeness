
pub struct Article {
    pub guid: Uuid,
    pub title: str,
    pub summary: str,
    pub body: str,
    pub date: SystemTime,
    pub author: str,
    pub link: str
}
