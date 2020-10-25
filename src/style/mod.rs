use crate::network::NetworkManager;
use crate::source::SourceCache;
use crate::style_spec;
use eyre::Result;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct Style {
    _style: style_spec::Style,
    _nm: Rc<NetworkManager>,
    _sources: HashMap<String, SourceCache>,
}

impl Style {
    pub async fn new(uri: &str, nm: Rc<NetworkManager>) -> Result<Self> {
        let style_str = nm.load_style(uri).await?;
        let style = serde_json::from_str::<style_spec::Style>(&style_str)?;

        let mut sources = HashMap::new();
        for (name, source) in &style.sources {
            sources.insert(
                name.clone(),
                SourceCache::new(nm.clone(), name, source).await?,
            );
        }

        Ok(Self {
            _style: style,
            _nm: nm,
            _sources: sources,
        })
    }
}
