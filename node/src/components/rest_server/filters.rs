use futures::FutureExt;
use http::Response;
use hyper::Body;
use tracing::{debug, warn};
use warp::{
    filters::BoxedFilter,
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

use super::{ReactorEventT, CLIENT_API_VERSION};
use crate::{
    effect::{requests::RestRequest, EffectBuilder},
    reactor::QueueKind,
    types::GetStatusResult,
};

/// The status URL path.
pub const STATUS_API_PATH: &str = "status";

/// The metrics URL path.
pub const METRICS_API_PATH: &str = "metrics";

pub(super) fn create_status_filter<REv: ReactorEventT>(
    effect_builder: EffectBuilder<REv>,
) -> BoxedFilter<(Response<Body>,)> {
    warp::get()
        .and(warp::path(STATUS_API_PATH))
        .and_then(move || {
            debug!("REST status request received");
            effect_builder
                .make_request(
                    |responder| RestRequest::GetStatus { responder },
                    QueueKind::Api,
                )
                .map(|status_feed| {
                    debug!("REST status retrieved {:?}", status_feed);

                    let mut body = GetStatusResult::from(status_feed);
                    body.set_api_version(CLIENT_API_VERSION.clone());
                    Ok::<_, Rejection>(reply::json(&body).into_response())
                })
        })
        .boxed()
}

pub(super) fn create_metrics_filter<REv: ReactorEventT>(
    effect_builder: EffectBuilder<REv>,
) -> BoxedFilter<(Response<Body>,)> {
    warp::get()
        .and(warp::path(METRICS_API_PATH))
        .and_then(move || {
            debug!("REST metrics request received");
            effect_builder
                .make_request(
                    |responder| RestRequest::GetMetrics { responder },
                    QueueKind::Api,
                )
                .map(|maybe_metrics| match maybe_metrics {
                    Some(metrics) => {
                        debug!("REST metrics retrieved");
                        Ok::<_, Rejection>(
                            reply::with_status(metrics, StatusCode::OK).into_response(),
                        )
                    }
                    None => {
                        warn!("metrics not available");
                        Ok(reply::with_status(
                            "metrics not available",
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                        .into_response())
                    }
                })
        })
        .boxed()
}