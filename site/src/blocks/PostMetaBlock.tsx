type Props = {
  props?: Readonly<Record<string, unknown>>;
};

function getString(value: unknown) {
  return typeof value === 'string' ? value : '';
}

export function PostMetaBlock({ props }: Props) {
  const author = getString(props?.author);
  const publishedAt = getString(props?.publishedAt);
  const updatedAt = getString(props?.updatedAt);
  const locale = getString(props?.locale) || 'fr-fr';

  if (!author && !publishedAt && !updatedAt) {
    return null;
  }

  const formatter = new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });

  return (
    <p className="post-meta">
      {author && <span>{author}</span>}

      {author && publishedAt && <span aria-hidden="true"> · </span>}

      {publishedAt && (
        <time dateTime={publishedAt}>
          {formatter.format(new Date(publishedAt))}
        </time>
      )}

      {updatedAt && updatedAt !== publishedAt && (
        <>
          <span aria-hidden="true"> · </span>
          <span>
            {locale === 'fr-fr' ? 'Mis à jour le ' : 'Updated on '}
            <time dateTime={updatedAt}>{formatter.format(new Date(updatedAt))}</time>
          </span>
        </>
      )}
    </p>
  );
}
