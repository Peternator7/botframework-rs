use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeroCard {
    #[serde(default)]
    pub buttons: Vec<super::actions::CardAction>,
    #[serde(default)]
    pub images: Vec<super::actions::CardImage>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,
}

impl HeroCard {
    pub fn new() -> HeroCard {
        HeroCard::default()
    }

    pub fn builder() -> HeroCardBuilder {
        HeroCardBuilder {
            inner: HeroCard::new()
        }
    }
}

pub struct HeroCardBuilder {
    inner: HeroCard
}

impl HeroCardBuilder {
    pub fn to_card(self) -> HeroCard {
        self.inner
    }

    pub fn title(mut self, title: &str) -> Self {
        self.inner.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.inner.subtitle = Some(subtitle.into());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.inner.text = Some(text.into());
        self
    }

    pub fn push_action(mut self, action: super::actions::CardAction) -> Self {
        self.inner.buttons.push(action);
        self
    }

    pub fn push_image(mut self, image: super::actions::CardImage) -> Self {
        self.inner.images.push(image);
        self
    }

    pub fn extend_actions<I>(mut self, actions: I) -> Self
        where I: std::iter::IntoIterator<Item = super::actions::CardAction>
    {
        self.inner.buttons.extend(actions);
        self
    }

    pub fn extend_images<I>(mut self, images: I) -> Self
        where I: std::iter::IntoIterator<Item = super::actions::CardImage>
    {
        self.inner.images.extend(images);
        self
    }
}
