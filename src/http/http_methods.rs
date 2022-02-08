use reqwest::Method as ReqwestMethod;

#[derive(Debug, Clone, Copy)]
pub enum Methods {
    Get,
    Post,
    Delete,
    Head,
    Put,
}

impl Methods {
    pub(crate) const fn as_reqwest(self) -> ReqwestMethod {
        match self {
            Self::Get => ReqwestMethod::GET,
            Self::Post => ReqwestMethod::POST,
            Self::Delete => ReqwestMethod::DELETE,
            Self::Head => ReqwestMethod::PATCH,
            Self::Put => ReqwestMethod::PUT,
        }
    }
}
