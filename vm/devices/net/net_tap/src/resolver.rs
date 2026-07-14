// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::TapEndpoint;
use crate::tap;
use net_backend::resolve::ResolveEndpointParams;
use net_backend::resolve::ResolvedEndpoint;
use net_backend_resources::tap::NamedTapHandle;
use net_backend_resources::tap::TapHandle;
use vm_resource::ResolveResource;
use vm_resource::declare_static_resolver;
use vm_resource::kind::NetEndpointHandleKind;

pub struct TapResolver;

declare_static_resolver! {
    TapResolver,
    (NetEndpointHandleKind, TapHandle),
    (NetEndpointHandleKind, NamedTapHandle),
}

impl ResolveResource<NetEndpointHandleKind, TapHandle> for TapResolver {
    type Output = ResolvedEndpoint;
    type Error = tap::Error;

    fn resolve(
        &self,
        resource: TapHandle,
        _input: ResolveEndpointParams,
    ) -> Result<Self::Output, Self::Error> {
        let tap = tap::Tap::new(resource.fd)?;

        Ok(TapEndpoint::new_named(tap, None)?.into())
    }
}

impl ResolveResource<NetEndpointHandleKind, NamedTapHandle> for TapResolver {
    type Output = ResolvedEndpoint;
    type Error = tap::Error;

    fn resolve(
        &self,
        resource: NamedTapHandle,
        _input: ResolveEndpointParams,
    ) -> Result<Self::Output, Self::Error> {
        let tap = tap::Tap::new(resource.fd)?;

        Ok(TapEndpoint::new_named(tap, Some(resource.name))?.into())
    }
}
