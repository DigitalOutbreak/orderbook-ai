import { promises as fs } from "node:fs"
import path from "node:path"

import { DOCS_PAGES, type DocsPage } from "./docs-config"

export type DocsHeading = {
  id: string
  text: string
  level: number
}

export type DocsContent = DocsPage & {
  content: string
  headings: DocsHeading[]
}

const docsDir = path.join(process.cwd(), "content", "docs")

function slugify(value: string) {
  return value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
}

function parseFrontmatter(source: string) {
  if (!source.startsWith("---\n")) {
    return {
      metadata: {} as Record<string, string>,
      content: source,
    }
  }

  const end = source.indexOf("\n---\n", 4)
  if (end === -1) {
    return {
      metadata: {} as Record<string, string>,
      content: source,
    }
  }

  const raw = source.slice(4, end)
  const content = source.slice(end + 5)
  const metadata = Object.fromEntries(
    raw
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean)
      .map((line) => {
        const index = line.indexOf(":")
        if (index === -1) return [line, ""]
        return [line.slice(0, index).trim(), line.slice(index + 1).trim()]
      })
  )

  return { metadata, content }
}

function extractHeadings(markdown: string): DocsHeading[] {
  return markdown
    .split("\n")
    .map((line) => line.match(/^(#{1,3})\s+(.*)$/))
    .filter((match): match is RegExpMatchArray => Boolean(match))
    .map((match) => ({
      level: match[1].length,
      text: match[2].trim(),
      id: slugify(match[2].trim()),
    }))
}

export function getDocsPages() {
  return DOCS_PAGES
}

export function getDocsPageBySlug(slug: string) {
  return DOCS_PAGES.find((page) => page.slug === slug)
}

export async function getDocsContent(slug: string): Promise<DocsContent | null> {
  const page = getDocsPageBySlug(slug)
  if (!page) return null

  const source = await fs.readFile(path.join(docsDir, page.file), "utf8")
  const { metadata, content } = parseFrontmatter(source)

  return {
    ...page,
    title: metadata.title || page.title,
    description: metadata.description || page.description,
    content,
    headings: extractHeadings(content),
  }
}

export function getDocsNeighbors(slug: string) {
  const index = DOCS_PAGES.findIndex((page) => page.slug === slug)
  return {
    previous: index > 0 ? DOCS_PAGES[index - 1] : null,
    next: index >= 0 && index < DOCS_PAGES.length - 1 ? DOCS_PAGES[index + 1] : null,
  }
}
