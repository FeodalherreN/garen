use reqwest::Method as ReqwestMethod;

#[derive(Debug, Clone, Copy)]
pub enum Methods {
    Get,
    Put,
}

impl Methods {
    pub(crate) const fn as_reqwest(self) -> ReqwestMethod {
        match self {
            Self::Get => ReqwestMethod::GET,
            Self::Put => ReqwestMethod::PUT,
        }
    }
}
