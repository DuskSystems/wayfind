use crate::util::PercentDecodedStr;
use http::Extensions;
use std::sync::Arc;
use wayfind::node::matches::Parameter;

#[derive(Clone)]
pub(crate) enum UrlParams {
    Params(Vec<(Arc<str>, PercentDecodedStr)>),
    InvalidUtf8InPathParam { key: Arc<str> },
}

pub(super) fn insert_url_params(extensions: &mut Extensions, params: &[Parameter]) {
    let current_params = extensions.get_mut();

    if let Some(UrlParams::InvalidUtf8InPathParam { .. }) = current_params {
        // nothing to do here since an error was stored earlier
        return;
    }

    let params = params
        .iter()
        .map(|param| {
            (
                String::from_utf8_lossy(param.key),
                String::from_utf8_lossy(param.value),
            )
        })
        .filter(|(key, _)| !key.starts_with(super::NEST_TAIL_PARAM))
        .filter(|(key, _)| !key.starts_with(super::FALLBACK_PARAM))
        .map(|(k, v)| {
            if let Some(decoded) = PercentDecodedStr::new(v) {
                Ok((Arc::from(k), decoded))
            } else {
                Err(Arc::from(k))
            }
        })
        .collect::<Result<Vec<_>, _>>();

    match (current_params, params) {
        (Some(UrlParams::InvalidUtf8InPathParam { .. }), _) => {
            unreachable!("we check for this state earlier in this method")
        }
        (_, Err(invalid_key)) => {
            extensions.insert(UrlParams::InvalidUtf8InPathParam { key: invalid_key });
        }
        (Some(UrlParams::Params(current)), Ok(params)) => {
            current.extend(params);
        }
        (None, Ok(params)) => {
            extensions.insert(UrlParams::Params(params));
        }
    }
}
