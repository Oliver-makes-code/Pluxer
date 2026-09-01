pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub footer: Option<String>,
    pub color: u32,
    pub fields: Vec<EmbedField>,
}

pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
