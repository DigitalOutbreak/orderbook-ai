import { notFound } from "next/navigation"

import { DocsLayout } from "@/components/docs/docs-layout"
import { getDocsContent, getDocsNeighbors, getDocsPages } from "@/lib/docs"

type DocsPageProps = {
  params: Promise<{
    slug: string
  }>
}

export async function generateStaticParams() {
  return getDocsPages().map((page) => ({ slug: page.slug }))
}

export default async function DocsPage({ params }: DocsPageProps) {
  const { slug } = await params
  const page = await getDocsContent(slug)

  if (!page) {
    notFound()
  }

  const pages = getDocsPages()
  const { previous, next } = getDocsNeighbors(slug)

  return <DocsLayout page={page} pages={pages} previous={previous} next={next} />
}
