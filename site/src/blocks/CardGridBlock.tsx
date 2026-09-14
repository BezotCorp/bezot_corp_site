type Props = {
  props?: Readonly<Record<string, unknown>>;
};

type Card = {
  title: string;
  text: string;
};

function isCard(value: unknown): value is Card {
  if (!value || typeof value !== "object") {
    return false;
  }

  const candidate = value as Record<string, unknown>;
  return typeof candidate.title === "string" && typeof candidate.text === "string";
}

export function CardGridBlock({ props }: Props) {
  const source =
    Array.isArray(props?.items) ? props.items : Array.isArray(props?.cards) ? props.cards : [];
  const items = source.filter(isCard);

  if (!items.length) {
    return null;
  }

  return (
    <section>
      {items.map((item) => (
        <article key={item.title}>
          <h2>{item.title}</h2>
          <p>{item.text}</p>
        </article>
      ))}
    </section>
  );
}
