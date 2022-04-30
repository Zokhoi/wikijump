/*
 * methods/page.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::models::page::Model as PageModel;
use crate::models::page_revision::Model as PageRevisionModel;
use crate::services::page::{
    CreatePage, DeletePage, EditPage, GetPageOutput, RestorePage,
};
use crate::services::{Result, TextService};
use crate::web::PageDetailsQuery;
use ref_map::*;
use std::borrow::Cow;

pub async fn page_invalid(req: ApiRequest) -> ApiResponse {
    tide::log::warn!("Received invalid /page path: {}", req.url());
    Ok(Response::new(StatusCode::BadRequest))
}

pub async fn page_head_direct(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let page_id = req.param("page_id")?.parse()?;
    tide::log::info!("Checking existence of page ID {page_id}");

    let exists = PageService::exists_direct(&ctx, page_id).await.to_api()?;
    txn.commit().await?;
    exists_status(exists)
}

pub async fn page_get_direct(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let page_id = req.param("page_id")?.parse()?;
    tide::log::info!("Getting page ID {page_id}");

    let details: PageDetailsQuery = req.query()?;
    let page = PageService::get_direct(&ctx, page_id).await.to_api()?;
    let revision = RevisionService::get_latest(&ctx, page.site_id, page.page_id)
        .await
        .to_api()?;

    let response = build_page_response(&ctx, &page, &revision, details, StatusCode::Ok)
        .await
        .to_api()?;
    txn.commit().await?;
    Ok(response)
}

pub async fn page_create(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let input: CreatePage = req.body_json().await?;
    let site_id = req.param("site_id")?.parse()?;
    tide::log::info!("Creating new page in site ID {site_id}");

    let output = PageService::create(&ctx, site_id, input).await.to_api()?;
    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

pub async fn page_head(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Checking existence of page {reference:?} in site ID {site_id}");

    let exists = PageService::exists(&ctx, site_id, reference)
        .await
        .to_api()?;

    txn.commit().await?;
    exists_status(exists)
}

pub async fn page_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let details: PageDetailsQuery = req.query()?;
    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Getting page {reference:?} in site ID {site_id}");

    let page = PageService::get(&ctx, site_id, reference).await.to_api()?;
    let revision = RevisionService::get_latest(&ctx, site_id, page.page_id)
        .await
        .to_api()?;

    let response = build_page_response(&ctx, &page, &revision, details, StatusCode::Ok)
        .await
        .to_api()?;

    txn.commit().await?;
    Ok(response)
}

pub async fn page_edit(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let input: EditPage = req.body_json().await?;
    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Editing page {reference:?} in site ID {site_id}");

    let output = PageService::edit(&ctx, site_id, reference, input)
        .await
        .to_api()?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_delete(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let input: DeletePage = req.body_json().await?;
    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Deleting page {reference:?} in site ID {site_id}");

    let output = PageService::delete(&ctx, site_id, reference, input)
        .await
        .to_api()?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_rerender(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let page_id = req.param("page_id")?.parse()?;
    tide::log::info!("Re-rendering page ID {page_id} in site ID {site_id}");

    RevisionService::rerender(&ctx, site_id, page_id)
        .await
        .to_api()?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn page_restore(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let input: RestorePage = req.body_json().await?;
    let site_id = req.param("site_id")?.parse()?;
    let page_id = req.param("page_id")?.parse()?;
    tide::log::info!("Un-deleting page ID {page_id} in site ID {site_id}");

    let output = PageService::restore(&ctx, site_id, page_id, input)
        .await
        .to_api()?;

    txn.commit().await?;
    let body = Body::from_json(&output)?;
    Ok(body.into())
}

pub async fn page_links_from_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Getting page links for page {reference:?} in site ID {site_id}");

    let page = PageService::get(&ctx, site_id, reference).await.to_api()?;
    let output = LinkService::get_from(&ctx, page.page_id).await.to_api()?;
    let body = Body::from_json(&output)?;
    txn.commit().await?;

    Ok(body.into())
}

pub async fn page_links_to_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!("Getting page links from page {reference:?} in site ID {site_id}");

    let page = PageService::get(&ctx, site_id, reference).await.to_api()?;
    let output = LinkService::get_to(&ctx, page.page_id, None)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;
    Ok(body.into())
}

pub async fn page_links_to_missing_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let page_slug = req.param("page_slug")?;
    tide::log::info!(
        "Getting missing page links from page slug {page_slug} in site ID {site_id}",
    );

    let output = LinkService::get_to_missing(&ctx, site_id, page_slug, None)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;
    Ok(body.into())
}

pub async fn page_links_external_from(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let reference = Reference::try_from(&req)?;
    tide::log::info!(
        "Getting external links from page {reference:?} in site ID {site_id}",
    );

    let page = PageService::get(&ctx, site_id, reference).await.to_api()?;
    let output = LinkService::get_external_from(&ctx, page.page_id)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;
    Ok(body.into())
}

pub async fn page_links_external_to(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let site_id = req.param("site_id")?.parse()?;
    let url = req.param("url")?;
    tide::log::info!("Getting external links to URL {url} in site ID {site_id}");

    let output = LinkService::get_external_to(&ctx, site_id, url)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    txn.commit().await?;
    Ok(body.into())
}

async fn build_page_response(
    ctx: &ServiceContext<'_>,
    page: &PageModel,
    revision: &PageRevisionModel,
    details: PageDetailsQuery,
    status: StatusCode,
) -> Result<Response> {
    // Get category slug from ID
    let category =
        CategoryService::get(ctx, page.site_id, Reference::from(page.page_category_id))
            .await?;

    // Get text data, if requested
    let (wikitext, compiled_html) = try_join!(
        TextService::get_maybe(ctx, details.wikitext, &revision.wikitext_hash),
        TextService::get_maybe(ctx, details.compiled_html, &revision.compiled_hash),
    )?;

    // Build result struct
    let output = GetPageOutput {
        page_id: page.page_id,
        page_created_at: page.created_at,
        page_updated_at: page.updated_at,
        page_deleted_at: page.deleted_at,
        page_revision_count: revision.revision_number + 1,
        site_id: page.site_id,
        page_category_id: category.category_id,
        page_category_slug: cow!(category.slug),
        discussion_thread_id: page.discussion_thread_id,
        revision_id: revision.revision_id,
        revision_type: revision.revision_type,
        revision_created_at: revision.created_at,
        revision_number: revision.revision_number,
        revision_user_id: revision.user_id,
        wikitext,
        compiled_html,
        compiled_at: revision.compiled_at,
        compiled_generator: cow!(revision.compiled_generator),
        revision_comments: cow!(revision.comments),
        hidden_fields: revision.hidden.clone(),
        title: cow!(revision.title),
        alt_title: cow_opt!(revision.alt_title),
        slug: cow!(revision.slug),
        tags: revision.tags.clone(),
    };

    let body = Body::from_json(&output)?;
    let response = Response::builder(status).body(body).into();
    Ok(response)
}
