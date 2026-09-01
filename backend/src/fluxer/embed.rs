use fluxer_types::{ApiEmbed, ApiEmbedField, ApiEmbedFooter, ApiEmbedMedia};

use crate::embed::{Embed, EmbedField};

impl From<Embed> for ApiEmbed {
    fn from(value: Embed) -> Self {
        return Self {
            kind: Some("rich".into()),
            title: value.title,
            description: value.description,
            footer: value.footer.map(|it| ApiEmbedFooter {
                text: it,
                icon_url: None,
                proxy_icon_url: None,
            }),
            color: Some(value.color),
            fields: Some(value.fields.into_iter().map(Into::into).collect()),
            thumbnail: value.thumbnail_url.map(|it| ApiEmbedMedia {
                url: it,
                proxy_url: None,
                content_hash: None,
                content_type: None,
                width: None,
                height: None,
                description: None,
                placeholder: None,
                duration: None,
                flags: None,
            }),

            url: None,
            timestamp: None,
            audio: None,
            author: None,
            image: None,
            provider: None,
            video: None,
            nsfw: None,
            children: None,
        };
    }
}

impl From<EmbedField> for ApiEmbedField {
    fn from(value: EmbedField) -> Self {
        return Self {
            name: value.name,
            value: value.value,
            inline: Some(value.inline),
        };
    }
}
