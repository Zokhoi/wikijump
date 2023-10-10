import { authGetSession } from "$lib/server/auth/getSession"
import * as page from "$lib/server/deepwell/page"

// Handling of server events from client

export async function POST(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let userSession = event.cookies.get("wikijump_token")
  let ipAddr = event.getClientAddress()
  let userAgent = event.cookies.get("User-Agent")

  let session = await authGetSession(userSession)

  let extra = event.params.extra
    ?.toLowerCase()
    .split("/")
    .filter((flag) => flag.length)

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteIdVal = data.get("site-id")?.toString()
  let siteId = siteIdVal ? parseInt(siteIdVal) : null

  let res: object = {}

  if (extra.includes("edit")) {
    /** Edit or create page. */
    let comments = data.get("comments")?.toString() ?? ""
    let wikitext = data.get("wikitext")?.toString()
    let title = data.get("title")?.toString()
    let altTitle = data.get("alt-title")?.toString()
    let tagsStr = data.get("tags")?.toString().trim()
    let tags: string[] = []
    if (tagsStr?.length) tags = tagsStr.split(" ").filter((tag) => tag.length)

    res = await page.pageEdit(
      siteId,
      pageId,
      session.user_id,
      slug,
      comments,
      wikitext,
      title,
      altTitle,
      tags
    )
  } else if (extra.includes("history")) {
    /** Retrieve page revision list. */
    let revisionNumberStr = data.get("revision-number")?.toString()
    let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
    let limitStr = data.get("limit")?.toString()
    let limit = limitStr ? parseInt(limitStr) : null

    res = await page.pageHistory(siteId, pageId, revisionNumber, limit)
  } else if (extra.includes("move")) {
    /** Move page to new slug. */
    let comments = data.get("comments")?.toString() ?? ""
    let newSlug = data.get("new-slug")?.toString()

    res = await page.pageMove(siteId, pageId, session.user_id, slug, newSlug, comments)
  } else if (extra.includes("revision")) {
    let revisionNumberStr = data.get("revision-number")?.toString()
    let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null

    res = await page.pageRevision(siteId, pageId, revisionNumber)
  }

  return new Response(JSON.stringify(res))
}

/** Delete page. */
export async function DELETE(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let userSession = event.cookies.get("wikijump_token")
  let ipAddr = event.getClientAddress()
  let userAgent = event.cookies.get("User-Agent")

  let session = await authGetSession(userSession)

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteIdVal = data.get("site-id")?.toString()
  let siteId = siteIdVal ? parseInt(siteIdVal) : null
  let comments = data.get("comments")?.toString() ?? ""

  let res = await page.pageDelete(siteId, pageId, session.user_id, slug, comments)
  return new Response(JSON.stringify(res))
}
