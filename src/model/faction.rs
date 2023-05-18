use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Faction {
    symbol: String,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<Trait>,
}

impl Faction {
    pub(crate) fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub(crate) fn headquarters(&self) -> &str {
        self.headquarters.as_ref()
    }

    pub(crate) fn traits(&self) -> &[Trait] {
        self.traits.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Trait {
    symbol: String,
    name: String,
    description: String,
}

impl Trait {
    pub(crate) fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct FactionResult {
    data: Vec<Faction>,
    meta: Meta,
}

impl FactionResult {
    pub(crate) fn data(&self) -> &[Faction] {
        self.data.as_ref()
    }

    pub(crate) fn meta(&self) -> &Meta {
        &self.meta
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Meta {
    total: isize,
    page: isize,
    limit: isize,
}

impl Meta {
    pub(crate) fn total(&self) -> isize {
        self.total
    }

    pub(crate) fn page(&self) -> isize {
        self.page
    }

    pub(crate) fn limit(&self) -> isize {
        self.limit
    }
}
