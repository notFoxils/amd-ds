use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    io::Read,
    str::FromStr,
};

use reqwest::{
    StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use snafu::{ResultExt, Snafu};

pub type RequestHeaders = HashMap<Box<str>, Box<str>>;

#[derive(Debug, Snafu)]
pub enum InvalidRequestHeaders {
    #[snafu(display("the provided header name '{header_name}' was invalid"))]
    InvalidHeaderName { header_name: Box<str> },
    #[snafu(display("the provided header value '{header_value}' was invalid"))]
    InvalidHeaderValue { header_value: Box<str> },
}

fn convert_to_headermap(
    request_headers: &RequestHeaders,
) -> Result<HeaderMap, InvalidRequestHeaders> {
    let mut header_map = HeaderMap::with_capacity(request_headers.len());
    for (header_name, header_value) in request_headers {
        let header_name = HeaderName::from_str(header_name).map_err(|_| {
            InvalidHeaderNameSnafu {
                header_name: header_value.clone(),
            }
            .build()
        })?;
        let header_value = HeaderValue::from_str(header_value).map_err(|_| {
            InvalidHeaderValueSnafu {
                header_value: header_value.clone(),
            }
            .build()
        })?;

        header_map.append(header_name, header_value);
    }

    Ok(header_map)
}

pub struct ReqwestError(reqwest::Error);

impl Debug for ReqwestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for ReqwestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for ReqwestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

#[derive(Debug, Snafu)]
pub enum RequestError {
    #[snafu(display("the provided request-headers were invalid"))]
    InvalidHeaders { source: InvalidRequestHeaders },
    #[snafu(display("failed to send the request"))]
    SendRequest { source: ReqwestError },
    #[snafu(display("recieved an invalid status code: {status_code}"))]
    InvalidStatusCode { status_code: StatusCode },
    #[snafu(display("recieved an invalid *sucess* code: {status_code}"))]
    InvalidSuccessCode { status_code: StatusCode },
}

pub fn request(
    request_url: &str,
    request_headers: &RequestHeaders,
) -> Result<impl Read, RequestError> {
    let request_headers = convert_to_headermap(request_headers).context(InvalidHeadersSnafu)?;

    let response = reqwest::blocking::Client::default()
        .get(request_url)
        .headers(request_headers)
        .send()
        .map_err(ReqwestError)
        .context(SendRequestSnafu)?;

    let response_status = response.status();
    if !response_status.is_success() {
        return InvalidStatusCodeSnafu {
            status_code: response_status,
        }
        .fail();
    } else if response_status != StatusCode::OK {
        return InvalidSuccessCodeSnafu {
            status_code: response_status,
        }
        .fail();
    }

    Ok(response)
}
