import { useEffect } from 'react';
import { Link } from 'react-router-dom';
import type { ContentBlock, SeoMetadata } from '../application/site-contract';
import { useSite } from '../application/use-site';
import { BlockRenderer } from '../blocks/BlockRenderer';
import { MainLayout } from '../layout/MainLayout';
import { applySeo } from '../seo';

type Props = {
  page: {
    seo: {
      title: string;
      description?: string;
    };
    blocks: readonly ContentBlock[];
  };
  seo: SeoMetadata;
};

function getBreadcrumbs(seo: SeoMetadata) {
  const canonicalPath = seo.canonicalPath ?? '/';
  const parts = canonicalPath.split('/').filter(Boolean);

  if (parts.length <= 1) {
    return [];
  }

  const locale = parts[0];
  const isFrench = locale === 'fr-fr';
  const isBlogPost = parts[1] === 'blog' && parts.length > 2;

  const breadcrumbs = [
    {
      label: isFrench ? 'Accueil' : 'Home',
      path: `/${locale}/`,
    },
  ];

  if (isBlogPost) {
    breadcrumbs.push({
      label: 'Blog',
      path: `/${locale}/blog/`,
    });
  }

  breadcrumbs.push({
    label: seo.ogTitle ?? seo.title,
    path: canonicalPath,
  });

  return breadcrumbs;
}

export function PageTemplate({ page, seo }: Props) {
  const site = useSite();

  useEffect(() => {
    applySeo(seo, site.metadata);
  }, [seo, site]);

  const breadcrumbs = getBreadcrumbs(seo);

  return (
    <MainLayout>
      <main className="page">
        {breadcrumbs.length > 0 && (
          <nav aria-label="Breadcrumb">
            <ol>
              {breadcrumbs.map((item, index) => {
                const isLast = index === breadcrumbs.length - 1;

                return (
                  <li key={item.path}>
                    {isLast ? <span aria-current="page">{item.label}</span> : <Link to={item.path}>{item.label}</Link>}
                  </li>
                );
              })}
            </ol>
          </nav>
        )}

        {page.blocks.map((block, index) => (
          <BlockRenderer key={`${block.type}-${index}`} block={block} />
        ))}
      </main>
    </MainLayout>
  );
}
