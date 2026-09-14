type Props = {
  props?: Readonly<Record<string, unknown>>;
};

function getString(value: unknown) {
  return typeof value === 'string' ? value.trim() : '';
}

export function AffiliateCalloutBlock({ props }: Props) {
  const title = getString(props?.title);
  const text = getString(props?.text);
  const url = getString(props?.url);
  const label = getString(props?.label);
  const disclosure = getString(props?.disclosure);

  if (!title || !url.startsWith('https://') || !label) {
    return null;
  }

  return (
    <aside className="affiliate-callout" aria-label={disclosure || title}>
      {disclosure && <p className="affiliate-callout__disclosure">{disclosure}</p>}
      <h2>{title}</h2>
      {text && <p>{text}</p>}
      <a href={url} rel="sponsored nofollow noopener noreferrer" target="_blank">
        {label}
      </a>
    </aside>
  );
}
