use crate::{RedirectUri, ResponseType, Scope, State, client::ClientId};

#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    response_type: ResponseType,
    client_id: ClientId,
    redirect_uri: Option<RedirectUri>,
    scope: Option<Scope>,
    state: Option<State>,
}

impl AuthorizationRequest {
    pub fn new(
        response_type: ResponseType,
        client_id: ClientId,
        redirect_uri: Option<RedirectUri>,
        scope: Option<Scope>,
        state: Option<State>,
    ) -> Self {
        Self {
            response_type,
            client_id,
            redirect_uri,
            scope,
            state,
        }
    }

    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }
    pub fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.redirect_uri.as_ref()
    }
    pub fn scope(&self) -> Option<&Scope> {
        self.scope.as_ref()
    }
    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::ValueObject;

    fn client_id() -> ClientId {
        ClientId::new("client-1".to_string()).unwrap()
    }
    fn redirect_uri() -> RedirectUri {
        RedirectUri::new("https://example.com/cb".to_string()).unwrap()
    }
    fn scope() -> Scope {
        Scope::new("read".to_string()).unwrap()
    }
    fn state() -> State {
        State::new("xyz".to_string()).unwrap()
    }

    #[test]
    fn getters_return_constructor_values() {
        let req = AuthorizationRequest::new(
            ResponseType::Code,
            client_id(),
            Some(redirect_uri()),
            Some(scope()),
            Some(state()),
        );
        assert_eq!(req.response_type(), &ResponseType::Code);
        assert_eq!(req.client_id(), &client_id());
        assert_eq!(req.redirect_uri(), Some(&redirect_uri()));
        assert_eq!(req.scope(), Some(&scope()));
        assert_eq!(req.state(), Some(&state()));
    }

    #[test]
    fn optional_fields_can_be_absent() {
        let req = AuthorizationRequest::new(ResponseType::Token, client_id(), None, None, None);
        assert_eq!(req.response_type(), &ResponseType::Token);
        assert!(req.redirect_uri().is_none());
        assert!(req.scope().is_none());
        assert!(req.state().is_none());
    }
}
