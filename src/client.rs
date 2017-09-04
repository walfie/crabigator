use error::*;
use futures::{Async, Future, Stream};
use hyper;
use model::{Response, UserInformation};
use serde::Deserialize;
use serde_json;
use std::borrow::Cow;

const API_BASE_URL: &'static str = "https://www.wanikani.com/api";
const DEFAULT_API_VERSION: &'static str = "v1.4";

pub struct Client<'a, C: 'a> {
    hyper_client: &'a hyper::Client<C>,
    api_key: Cow<'a, str>,
    api_version: Cow<'a, str>,
}

// TODO: Maybe not always 'static
pub struct FutureResponse<T> {
    future: Box<Future<Item = Response<'static, T>, Error = Error>>,
}

impl<T> Future for FutureResponse<T> {
    type Item = Response<'static, T>;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        self.future.poll()
    }
}

impl<'client, C: 'client> Client<'client, C>
where
    C: hyper::client::Connect,
{
    pub fn new<S>(hyper_client: &'client hyper::Client<C>, api_key: S) -> Client<'client, C>
    where
        S: Into<Cow<'client, str>>,
    {
        Client {
            hyper_client,
            api_key: api_key.into(),
            api_version: DEFAULT_API_VERSION.into(),
        }
    }

    fn request(&self, resource: &str, options: Option<&str>) -> Result<hyper::client::Request> {
        let unparsed_uri = format!(
            "{}/user/{}/{}/{}",
            API_BASE_URL,
            self.api_key,
            resource,
            options.unwrap_or("")
        );

        let uri = unparsed_uri.parse().chain_err(
            || ErrorKind::Uri(unparsed_uri),
        )?;

        Ok(hyper::client::Request::new(hyper::Method::Get, uri))
    }

    fn send<T>(&self, request: hyper::client::Request) -> FutureResponse<T>
    where
        T: ::serde::de::DeserializeOwned + 'static,
    {
        let response = self.hyper_client
            .request(request)
            .and_then(|res| res.body().concat2())
            .then(|res| res.chain_err(|| ErrorKind::Http))
            .and_then(|bytes| {
                serde_json::from_slice(&bytes).chain_err(|| ErrorKind::Deserialize(bytes.to_vec()))
            });

        FutureResponse { future: Box::new(response) }
    }

    pub fn user_information(&self) -> FutureResponse<()> {
        // TODO: Don't unwrap
        self.send(self.request("user-information", None).unwrap())
    }
}
