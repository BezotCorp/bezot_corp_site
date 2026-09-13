import type { ComponentType } from 'react';

export type SiteMetadata = {
  readonly name: string;
  readonly baseUrl: string;
  readonly defaultLocale: string;
  readonly locales: readonly string[];
  readonly defaultOgImage: string;
};

export type ContentSeo = {
  readonly title: string;
  readonly description?: string;
  readonly ogTitle: string;
  readonly ogDescription?: string;
  readonly ogImage?: string;
};

export type ContentBlock = {
  readonly type: string;
  readonly props?: Readonly<Record<string, unknown>>;
};

export type BlockComponentProps = {
  readonly props?: Readonly<Record<string, unknown>>;
};

export type BlockRegistry = Readonly<Record<string, ComponentType<BlockComponentProps>>>;

export type SiteEntryContent = {
  readonly slug: string;
  readonly status?: string;
  readonly updatedAt?: string;
  readonly seo: ContentSeo;
  readonly blocks: readonly ContentBlock[];
};

export type SitePage = {
  readonly id: string;
  readonly locales: Readonly<Record<string, SiteEntryContent>>;
};

export type SitePost = {
  readonly id: string;
  readonly type: string;
  readonly status: string;
  readonly author?: string;
  readonly publishedAt?: string;
  readonly updatedAt?: string;
  readonly locales: Readonly<Record<string, SiteEntryContent>>;
};

export type SiteContent = {
  readonly site: SiteMetadata;
  readonly redirects: unknown;
  readonly gone: unknown;
  readonly pages: readonly SitePage[];
  readonly posts: readonly SitePost[];
};

export type SiteEntry = SitePage | SitePost;

export type SeoAlternate = {
  readonly locale: string;
  readonly path: string;
};

export type SeoMetadata = {
  readonly title: string;
  readonly description?: string;
  readonly robots: string;
  readonly lang: string;
  readonly canonicalPath?: string;
  readonly alternates: readonly SeoAlternate[];
  readonly ogTitle: string;
  readonly ogDescription?: string;
  readonly ogImage: string;
};

export type RouteMatch =
  | {
      readonly kind: 'root';
      readonly locale: string;
      readonly path: '/';
      readonly seo: SeoMetadata;
    }
  | {
      readonly kind: 'page';
      readonly locale: string;
      readonly path: string;
      readonly entry: SiteEntry;
      readonly content: SiteEntryContent;
      readonly seo: SeoMetadata;
    }
  | {
      readonly kind: 'not-found';
      readonly locale: string;
      readonly path: string;
      readonly seo: SeoMetadata;
    };

export type PrerenderRoute = {
  readonly path: string;
  readonly seo: SeoMetadata;
};

export interface SiteRuntime {
  readonly content: SiteContent;
  readonly blocks: BlockRegistry;
  readonly metadata: SiteMetadata;
  readonly defaultLocale: string;
  isLocale(value: string | undefined): boolean;
  getActiveLocale(pathname: string): string;
  isPostEntry(entry: SiteEntry): entry is SitePost;
  resolveRoute(pathname: string): RouteMatch;
  getPagePath(pageId: string, locale: string): string | null;
  getPostPath(postId: string, locale: string): string | null;
  getPublishedPosts(): readonly SitePost[];
  getPathForLocaleAndSlug(locale: string, slug: string): string;
  getNotFoundSeo(locale: string): SeoMetadata;
  getPrerenderRoutes(): readonly PrerenderRoute[];
}
