use error::*;
use futures::{Async, Future, Stream};
use hyper;
use model::{self, Response};
use serde::de::DeserializeOwned;
use serde_json;
use std::borrow::Cow;
use std::fmt;

const API_BASE_URL: &'static str = "https://www.wanikani.com/api";
const DEFAULT_API_VERSION: &'static str = "v1.4";

pub struct Client<'a, C: 'a> {
    hyper_client: &'a hyper::Client<C>,
    api_key: Cow<'a, str>,
    api_version: Cow<'a, str>,
}

pub struct FutureResponse<T>(Box<Future<Item = Response<T>, Error = Error>>);

impl<T> Future for FutureResponse<T> {
    type Item = Response<T>;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        self.0.poll()
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

    pub fn user_information(&self) -> FutureResponse<()> {
        self.get("user-information")
    }

    pub fn study_queue(&self) -> FutureResponse<model::StudyQueue> {
        self.get("study-queue")
    }

    pub fn level_progression(&self) -> FutureResponse<model::LevelProgression> {
        self.get("level-progression")
    }

    pub fn srs_distribution(&self) -> FutureResponse<model::SrsDistribution> {
        self.get("srs-distribution")
    }

    pub fn recent_unlocks(&self, limit: Option<u8>) -> FutureResponse<Vec<model::RecentUnlock>> {
        self.get_with_options("recent-unlocks", limit)
    }

    pub fn critical_items(
        &self,
        max_percentage: Option<u8>,
    ) -> FutureResponse<Vec<model::CriticalItem>> {
        self.get_with_options("critical-items", max_percentage)
    }

    pub fn radicals(&self, levels: Option<&[model::Level]>) -> FutureResponse<Vec<model::Radical>> {
        self.get_with_options("radicals", levels.map(DisplayableSlice))
    }

    pub fn kanji(&self, levels: Option<&[model::Level]>) -> FutureResponse<Vec<model::Kanji>> {
        self.get_with_options("kanji", levels.map(DisplayableSlice))
    }

    pub fn vocabulary(
        &self,
        levels: Option<&[model::Level]>,
    ) -> FutureResponse<Vec<model::Vocabulary>> {
        self.get_with_options("vocabulary", levels.map(DisplayableSlice))
    }

    fn request<O>(&self, resource: &str, options: Option<O>) -> Result<hyper::client::Request>
    where
        O: fmt::Display,
    {
        let unparsed_uri = format!(
            "{}/{}/user/{}/{}/{}",
            API_BASE_URL,
            self.api_version,
            self.api_key,
            resource,
            DisplayableOption(options)
        );

        let uri = unparsed_uri.parse().chain_err(
            || ErrorKind::Uri(unparsed_uri),
        )?;

        Ok(hyper::client::Request::new(hyper::Method::Get, uri))
    }

    fn send_request<T>(&self, request: Result<hyper::client::Request>) -> FutureResponse<T>
    where
        T: DeserializeOwned + 'static,
    {
        match request {
            Ok(req) => {
                let resp = self.hyper_client
                    .request(req)
                    .and_then(|res| res.body().concat2())
                    .then(|res| res.chain_err(|| ErrorKind::Http))
                    .and_then(|bytes| {
                        serde_json::from_slice(&bytes).chain_err(|| {
                            ErrorKind::Deserialize(bytes.to_vec())
                        })
                    });

                FutureResponse(Box::new(resp))
            }
            Err(e) => FutureResponse(Box::new(::futures::future::err(e))),

        }
    }

    fn get<T>(&self, resource: &str) -> FutureResponse<T>
    where
        T: DeserializeOwned + 'static,
    {
        let none: Option<String> = None;
        self.send_request(self.request(resource, none))
    }

    fn get_with_options<O, T>(&self, resource: &str, options: Option<O>) -> FutureResponse<T>
    where
        O: fmt::Display,
        T: DeserializeOwned + 'static,
    {
        self.send_request(self.request(resource, options))
    }
}

struct DisplayableOption<T>(Option<T>);
impl<T> fmt::Display for DisplayableOption<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref v) => v.fmt(f),
            None => Ok(()),
        }
    }
}

struct DisplayableSlice<'a, T: 'a>(&'a [T]);
impl<'a, T> fmt::Display for DisplayableSlice<'a, T>
where
    T: fmt::Display + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self.0 {
            write!(f, "{},", item)?
        }
        Ok(())
    }
}
